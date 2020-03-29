use url::Url;
use value_trait::*;
use super::validators;

pub struct Ref;

impl<V> super::Keyword<V> for Ref
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
    //fn compile(&self) -> Result<bool, schema::SchemaError> {
    fn compile(&self) -> super::KeywordResult<V> {
        let url = Url::options().parse("https://google.com").unwrap();
        Ok(Some(Box::new(validators::Ref { url })))
    }
}
