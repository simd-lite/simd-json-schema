use std::fmt;
use std::sync::Arc;
use value_trait::Value;
use std::any;

use super::schema;
use super::validators;

pub mod ref_;

pub type KeywordPair<V> = (Vec<String>, Box<dyn Keyword<V>>);
pub type KeywordMap<V> = hashbrown::HashMap<String, Arc<KeywordConsumer<V>>>;
pub type KeywordResult<V> = Result<Option<validators::BoxedValidator<V>>, schema::SchemaError>;
//pub type KeywordMap<V> = hashbrown::HashMap<String, Box<dyn Keyword<V>>>;
//pub type KeywordMap<V> = hashbrown::HashMap<String, Box<dyn Keyword<V>>>;
//pub type KeywordMap<V> = hashbrown::HashMap<String, V>;
//pub type KeywordMap<V> = hashbrown::HashMap<String, KeywordConsumer<V>>;

pub trait Keyword<V>: Send + Sync + any::Any {
    //fn compile(&self) -> Result<bool, schema::SchemaError> {
    fn compile(&self) -> KeywordResult<V>;
    fn is_exclusive(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct KeywordConsumer<V>
where
    V: Value,
{
    pub keys: Vec<String>,
    pub keyword: Box<dyn Keyword<V>>,
}

impl<V> KeywordConsumer<V>
where
    V: Value,
{
    pub fn consume(&self, set: &mut hashbrown::HashSet<String>) {
        for key in self.keys.iter() {
            if set.contains(key) {
                set.remove(key);
            }
        }
    }
}

impl<V> fmt::Debug for dyn Keyword<V> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("<keyword>")
    }
}

//pub fn default<'scope, V: 'scope>() -> hashbrown::HashMap<String, V>
//pub fn default<'data, V: 'data>() -> hashbrown::HashMap<String, V>
pub fn default<V>() -> KeywordMap<V>
where
    V: Value
        + std::clone::Clone
        + std::convert::From<simd_json::value::owned::Value>
        + std::fmt::Display
        + std::marker::Sync
        + std::marker::Send
        + std::cmp::PartialEq,
    <V as Value>::Key: std::borrow::Borrow<str>
        + std::hash::Hash
        + Eq
        + std::convert::AsRef<str>
        + std::fmt::Debug
        + std::string::ToString
        + std::marker::Sync
        + std::marker::Send,
{
    let mut map = hashbrown::HashMap::new();

    decouple_keyword((vec!["$ref".to_string()], Box::new(ref_::Ref)), &mut map);

    map
}

//pub fn decouple_keyword<V>(keyword_pair: KeywordPair<V>, map: &mut KeywordMap<V>)
pub fn decouple_keyword<V>(keyword_pair: KeywordPair<V>, map: &mut hashbrown::HashMap<String, Arc<KeywordConsumer<V>>>)
where
    V: Value,
{
    let (keys, keyword) = keyword_pair;

    let consumer = Arc::new(KeywordConsumer {
        keys: keys.clone(),
        keyword,
    });

    for key in keys.iter() {
        map.insert(key.to_string(), consumer.clone());
    }
}
