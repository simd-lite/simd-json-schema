use super::helpers;
use super::keywords;
use super::schema;
use hashbrown::HashMap;
use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

#[derive(Debug)]
pub struct Scope<V>
where
    V: Value,
{
    keywords: keywords::KeywordMap<V>,
    schemes: HashMap<String, schema::Schema<V>>,
}

impl<V> Scope<V>
where
    V: Value,
{
    pub fn new<'scope>() -> Scope<V>
    where
        V: Value
            + std::convert::From<simd_json::value::owned::Value>
            + std::clone::Clone
            + std::marker::Sync
            + std::marker::Send
            + std::cmp::PartialEq
            + std::fmt::Display,
        <V as Value>::Key: std::borrow::Borrow<str>
            + std::convert::AsRef<str>
            + std::fmt::Display
            + std::marker::Sync
            + std::marker::Send
            + std::fmt::Debug,
    {
        let scope = Scope {
            keywords: keywords::default(),
            schemes: HashMap::new(),
        };

        //scope.add_keyword(vec!["format".to_string()], keywords::format::Format::new());

        scope
    }

    pub fn resolve(&self, id: &url::Url) -> Option<schema::ScopedSchema<V>>
    where
        <V as Value>::Key: std::borrow::Borrow<str>
            + std::hash::Hash
            + Eq
            + std::convert::AsRef<str>
            + std::fmt::Debug
            + std::string::ToString,
    {
        let (schema_path, fragment) = helpers::serialize_schema_path(id);

        let schema = self.schemes.get(&schema_path).or_else(|| {
            for (_, schema) in self.schemes.iter() {
                let internal_schema = schema.resolve(schema_path.as_ref());
                if internal_schema.is_some() {
                    return internal_schema;
                }
            }

            None
        });

        schema.and_then(|schema| match fragment {
            Some(ref fragment) => schema
                .resolve_fragment(fragment)
                .map(|schema| schema::ScopedSchema::new(self, &*schema)),
            None => Some(schema::ScopedSchema::new(self, &*schema)),
        })
    }

    pub fn compile_and_return<'scope>(
        &'scope mut self,
        def: OwnedValue,
        ban_unknown: bool,
    ) -> Result<schema::ScopedSchema<'scope, 'scope, V>, schema::SchemaError>
    where
        V: Value + std::convert::From<simd_json::value::owned::Value> + std::clone::Clone,
        <V as Value>::Key: std::borrow::Borrow<str>
            + std::convert::AsRef<str>
            + std::fmt::Display
            + std::fmt::Debug,
    {
        let schema = schema::compile(
            def,
            None,
            schema::CompilationSettings::new(self.keywords.clone(), ban_unknown),
        )?;
        self.add_and_return(schema.id.clone().as_ref().unwrap(), schema)
    }

    fn add_and_return<'scope>(
        &'scope mut self,
        id: &url::Url,
        schema: schema::Schema<V>,
    ) -> Result<schema::ScopedSchema<'scope, 'scope, V>, schema::SchemaError> {
        let (id_str, fragment) = helpers::serialize_schema_path(id);
        dbg!(id_str.clone());
        dbg!(fragment.clone());

        if fragment.is_some() {
            return Err(schema::SchemaError::WrongId);
        }

        if !self.schemes.contains_key(&id_str) {
            self.schemes.insert(id_str.clone(), schema);
            Ok(schema::ScopedSchema::new(self, &self.schemes[&id_str]))
        } else {
            Err(schema::SchemaError::IdConflicts)
        }
    }

    pub fn add_keyword<T>(&mut self, keys: Vec<String>, keyword: T)
    where
        T: keywords::Keyword<V>,
    {
        keywords::decouple_keyword((keys, Box::new(keyword)), &mut self.keywords);
    }
}
