use super::helpers;
use super::keywords;
use super::schema;
use hashbrown::HashMap;
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
    pub fn new<'scope, 'schema: 'scope, 'data: 'schema>() -> Scope<V>
    where
        V: Value + std::convert::From<simd_json::value::owned::Value> + std::clone::Clone + std::marker::Sync + std::marker::Send + std::cmp::PartialEq + std::fmt::Display,
        <V as Value>::Key: std::borrow::Borrow<str> + std::convert::AsRef<str> + std::fmt::Display + std::marker::Sync + std::marker::Send + std::fmt::Debug,
    {
        let scope = Scope {
            keywords: keywords::default(),
            schemes: HashMap::new(),
        };

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

    pub fn compile_and_return<'scope, 'schema: 'scope, 'data: 'schema>(
        &'scope mut self,
        def: V,
        ban_unknown: bool,
    ) -> Result<schema::ScopedSchema<'scope, 'scope, V>, schema::SchemaError>
    where
        V: Value + std::convert::From<simd_json::value::owned::Value> + std::clone::Clone,
        <V as Value>::Key: std::borrow::Borrow<str> + std::convert::AsRef<str> + std::fmt::Display + std::fmt::Debug,
    {
        let schema = schema::compile(def, None, schema::CompilationSettings::new(self.keywords.clone(), ban_unknown))?;
        self.add_and_return(schema)
    }

    fn add_and_return<'scope, 'schema: 'scope>(
        &'scope mut self,
        schema: schema::Schema<V>,
    ) -> Result<schema::ScopedSchema<'scope, 'scope, V>, schema::SchemaError> {
        let id_str = "ernad".to_string();
        self.schemes.insert(id_str.clone(), schema);
        Ok(schema::ScopedSchema::new(self, &self.schemes[&id_str]))
    }
}
