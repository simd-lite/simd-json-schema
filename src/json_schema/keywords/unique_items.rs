use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

use super::super::schema;
use super::super::validators;

#[allow(missing_copy_implementations)]
pub struct UniqueItems;
impl<V: std::string::ToString> super::Keyword<V> for UniqueItems
where
    V: Value
        + std::clone::Clone
        + std::convert::From<simd_json::value::owned::Value>
        + std::fmt::Display,
    <V as Value>::Key: std::borrow::Borrow<str>
        + std::hash::Hash
        + Eq
        + std::convert::AsRef<str>
        + std::fmt::Debug
        + std::string::ToString
        + std::marker::Sync
        + std::marker::Send,
{
    fn compile(
        &self,
        def: &OwnedValue,
        ctx: &schema::WalkContext<'_>,
    ) -> super::KeywordCompilationResult<V> {
        let uniq = keyword_key_exists!(def, "uniqueItems");

        if uniq.is_bool() {
            if uniq.as_bool().unwrap() {
                Ok(Some(Box::new(validators::UniqueItems)))
            } else {
                Ok(None)
            }
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.join("/"),
                detail: "The value of pattern must be boolean".to_string(),
            })
        }
    }
}
