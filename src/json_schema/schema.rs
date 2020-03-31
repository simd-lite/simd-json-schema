use super::helpers;
use super::keywords;
use super::scope;
use super::validators;

use phf;

use std::collections;

use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

#[derive(Debug)]
pub struct Schema<V>
where
    V: Value,
{
    pub id: Option<url::Url>,
    schema: Option<url::Url>,
    // JSON that defines schema
    source: OwnedValue,
    tree: collections::BTreeMap<String, Schema<V>>,
    validators: validators::Validators<V>,
    scopes: hashbrown::HashMap<String, Vec<String>>,
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub struct ScopedSchema<'scope, 'schema: 'scope, V>
where
    V: Value,
{
    scope: &'scope scope::Scope<V>,
    schema: &'schema Schema<V>,
}

pub struct CompilationSettings<V>
where
    V: Value,
{
    pub keywords: keywords::KeywordMap<V>,
    pub ban_unknown_keywords: bool,
}

impl<V> CompilationSettings<V>
where
    V: Value,
{
    pub fn new(
        keywords: keywords::KeywordMap<V>,
        ban_unknown_keywords: bool,
    ) -> CompilationSettings<V> {
        CompilationSettings {
            keywords,
            ban_unknown_keywords,
        }
    }
}

#[derive(Debug)]
pub enum SchemaError {
    WrongId,
    IdConflicts,
    NotAnObject,
    UrlParseError(url::ParseError),
    UnknownKey(String),
    Malformed { path: String, detail: String },
}

#[derive(Debug)]
pub struct WalkContext<'walk> {
    pub url: &'walk url::Url,
    pub fragment: Vec<String>,
    pub scopes: &'walk mut hashbrown::HashMap<String, Vec<String>>,
}

impl<'walk> WalkContext<'walk> {
    pub fn escaped_fragment(&self) -> String {
        helpers::connect(
            self.fragment
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<&str>>()
                .as_ref(),
        )
    }
}

impl<'scope, 'schema, V> ScopedSchema<'scope, 'schema, V>
where
    V: Value,
{
    pub fn new(
        scope: &'scope scope::Scope<V>,
        schema: &'schema Schema<V>,
    ) -> ScopedSchema<'scope, 'schema, V> {
        ScopedSchema {
            scope,
            schema: &schema,
        }
    }

    pub fn validate(&self, data: &V) -> validators::ValidationState
    where
        V: Value + std::fmt::Debug,
        <V as Value>::Key: std::borrow::Borrow<str> + std::convert::AsRef<str> + std::fmt::Debug,
    {
        dbg!(data.clone());
        self.schema.validate_in_scope(data, "", &self.scope)
    }

    pub fn validate_in(&self, data: &V, path: &str) -> validators::ValidationState
    where
        <V as Value>::Key: std::borrow::Borrow<str> + std::convert::AsRef<str> + std::fmt::Debug,
    {
        self.schema.validate_in_scope(data, path, &self.scope)
    }
}

impl<V> Schema<V>
where
    V: Value,
    <V as Value>::Key: std::borrow::Borrow<str> + std::convert::AsRef<str> + std::fmt::Debug,
{
    fn validate_in_scope(
        &self,
        data: &V,
        path: &str,
        scope: &scope::Scope<V>,
    ) -> validators::ValidationState {
        let mut state = validators::ValidationState::new();

        for validator in self.validators.iter() {
            println!("VALIDATOR");
            state.append(validator.validate(data, path, scope))
        }

        state
    }
    pub fn resolve(&self, id: &str) -> Option<&Schema<V>> {
        let path = self.scopes.get(id);
        path.map(|path| {
            let mut schema = self;
            for item in path.iter() {
                schema = &schema.tree[item]
            }
            schema
        })
    }

    pub fn resolve_fragment(&self, fragment: &str) -> Option<&Schema<V>> {
        assert!(fragment.starts_with('/'), "Can't resolve id fragments");

        let parts = fragment[1..].split('/');
        let mut schema = self;
        for part in parts {
            match schema.tree.get(part) {
                Some(sch) => schema = sch,
                None => return None,
            }
        }

        Some(schema)
    }

    fn compile(
        source: OwnedValue,
        external_id: Option<url::Url>,
        settings: CompilationSettings<V>,
    ) -> Result<Schema<V>, SchemaError>
    where
        V: Value + std::convert::From<simd_json::value::owned::Value> + std::clone::Clone,
        <V as Value>::Key: std::borrow::Borrow<str>
            + std::convert::AsRef<str>
            + std::string::ToString
            + std::fmt::Debug,
    {
        let source = helpers::convert_boolean_schema(source);
        dbg!(source.clone());

        if !source.is_object() {
            return Err(SchemaError::NotAnObject);
        }

        let id = if external_id.is_some() {
            external_id.unwrap()
        } else {
            helpers::parse_url_key("$id", &source)?
                .clone()
                .unwrap_or_else(helpers::generate_id)
        };

        let schema = helpers::parse_url_key("$schema", &source)?;

        let (tree, mut scopes) = {
            let mut tree = collections::BTreeMap::new();
            let obj = source.as_object().unwrap();

            let mut scopes = hashbrown::HashMap::new();

            for (key, val) in obj.iter() {
                dbg!(key);
                if !val.is_object() && !val.is_array() && !val.is_bool() {
                    continue;
                }

                if FINAL_KEYS.contains(&key[..]) {
                    continue;
                }

                let mut context = WalkContext {
                    url: &id,
                    // NOTE: ToString bound stems from here
                    fragment: vec![key.to_string().clone()],
                    scopes: &mut scopes,
                };

                let scheme = Schema::compile_sub(
                    val.clone(),
                    &mut context,
                    &settings,
                    !NON_SCHEMA_KEYS.contains(key.as_str()),
                )?;

                tree.insert(helpers::encode(key.as_ref()), scheme);
            }

            (tree, scopes)
        };

        let validators = Schema::compile_keywords(
            source.clone(),
            &WalkContext {
                url: &id,
                fragment: vec![],
                scopes: &mut scopes,
            },
            &settings,
        )?;

        let schema = Schema {
            id: Some(id),
            schema,
            source,
            tree,
            validators,
            scopes,
        };

        Ok(schema)
    }

    fn compile_keywords<'key>(
        source: OwnedValue,
        context: &WalkContext<'key>,
        settings: &CompilationSettings<V>,
    ) -> Result<validators::Validators<V>, SchemaError>
    where
        V: Value + std::convert::From<simd_json::value::owned::Value> + std::clone::Clone,
        <V as Value>::Key: std::borrow::Borrow<str>
            + std::hash::Hash
            + Eq
            + std::convert::AsRef<str>
            + std::fmt::Debug
            + std::string::ToString,
    {
        let mut validators = vec![];
        let mut keys: hashbrown::HashSet<String> = source
            .as_object()
            .unwrap()
            .keys()
            .map(|key| key.to_string())
            .collect();
        let mut not_consumed = hashbrown::HashSet::new();
        dbg!(source.clone());
        dbg!(keys.clone());

        loop {
            let key = keys.iter().next().cloned();
            if key.is_some() {
                let key = key.unwrap();
                match settings.keywords.get(&key) {
                    Some(keyword) => {
                        keyword.consume(&mut keys);

                        let is_exclusive_keyword = keyword.keyword.is_exclusive();

                        if let Some(validator) = keyword.keyword.compile(&source, context)? {
                            if is_exclusive_keyword {
                                validators = vec![validator];
                            } else {
                                validators.push(validator);
                            }
                        }

                        if is_exclusive_keyword {
                            break;
                        }
                    }
                    None => {
                        keys.remove(&key);
                        if settings.ban_unknown_keywords {
                            not_consumed.insert(key);
                        }
                    }
                }
            } else {
                break;
            }
        }

        if settings.ban_unknown_keywords && !not_consumed.is_empty() {
            for key in not_consumed.iter() {
                if !ALLOW_NON_CONSUMED_KEYS.contains(&key[..]) {
                    return Err(SchemaError::UnknownKey(key.to_string()));
                }
            }
        }

        Ok(validators)
    }

    fn compile_sub(
        source: OwnedValue,
        context: &mut WalkContext<'_>,
        settings: &CompilationSettings<V>,
        is_schema: bool,
    ) -> Result<Schema<V>, SchemaError>
    where
        V: Value + std::convert::From<simd_json::value::owned::Value> + std::clone::Clone,
        <V as Value>::Key:
            std::borrow::Borrow<str> + std::convert::AsRef<str> + std::string::ToString,
    {
        let source = helpers::convert_boolean_schema(source);

        let id = if is_schema {
            helpers::parse_url_key_with_base("$id", &source, context.url)?
        } else {
            None
        };

        let schema = if is_schema {
            helpers::parse_url_key("$schema", &source)?
        } else {
            None
        };

        let tree = {
            let mut tree = collections::BTreeMap::new();

            if source.is_object() {
                let obj = source.as_object().unwrap();
                let parent_key = &context.fragment[context.fragment.len() - 1];

                for (key, val) in obj.iter() {
                    if !val.is_object() && !val.is_array() && !val.is_bool() {
                        continue;
                    }

                    if !PROPERTY_KEYS.contains(&parent_key[..]) && FINAL_KEYS.contains(&key[..]) {
                        continue;
                    }

                    let mut current_fragment = context.fragment.clone();
                    current_fragment.push(key.to_string().clone());

                    let is_schema = PROPERTY_KEYS.contains(&parent_key[..])
                        || !NON_SCHEMA_KEYS.contains(&key[..]);

                    let mut context = WalkContext {
                        url: id.as_ref().unwrap_or(context.url),
                        fragment: current_fragment,
                        scopes: context.scopes,
                    };

                    let scheme =
                        Schema::compile_sub(val.clone(), &mut context, &settings, is_schema)?;

                    tree.insert(helpers::encode(key.as_ref()), scheme);
                }
            } else if source.is_array() {
                let array = source.as_array().unwrap();
                let parent_key = &context.fragment[context.fragment.len() - 1];

                for (idx, val) in array.iter().enumerate() {
                    let mut val = val.clone();

                    if BOOLEAN_SCHEMA_ARRAY_KEYS.contains(&parent_key[..]) {
                        val = helpers::convert_boolean_schema(val);
                    }

                    if !val.is_object() && !val.is_array() {
                        continue;
                    }

                    let mut current_fragment = context.fragment.clone();
                    current_fragment.push(idx.to_string().clone());

                    let mut context = WalkContext {
                        url: id.as_ref().unwrap_or(context.url),
                        fragment: current_fragment,
                        scopes: context.scopes,
                    };

                    let scheme = Schema::compile_sub(val.clone(), &mut context, settings, true)?;

                    tree.insert(idx.to_string().clone(), scheme);
                }
            }
            tree
        };

        if id.is_some() {
            context
                .scopes
                .insert(id.clone().unwrap().into_string(), context.fragment.clone());
        }

        let validators = if is_schema && source.is_object() {
            Schema::compile_keywords(source.clone(), context, settings)?
        } else {
            vec![]
        };

        let schema = Schema {
            id: id,
            schema,
            source,
            tree,
            validators,
            scopes: hashbrown::HashMap::new(),
        };

        Ok(schema)
    }
}

pub fn compile<V>(
    source: OwnedValue,
    external_id: Option<url::Url>,
    settings: CompilationSettings<V>,
) -> Result<Schema<V>, SchemaError>
where
    V: Value + std::convert::From<simd_json::value::owned::Value> + std::clone::Clone,
    <V as Value>::Key:
        std::borrow::Borrow<str> + std::convert::AsRef<str> + std::fmt::Display + std::fmt::Debug,
{
    Schema::compile(source, external_id, settings)
}
