use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

use super::helpers;
use super::schema;
use super::validators;

#[allow(missing_copy_implementations)]
pub struct Not;
impl<V> super::Keyword<V> for Not
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
    fn compile(
        &self,
        def: &OwnedValue,
        ctx: &schema::WalkContext<'_>,
    ) -> super::KeywordCompilationResult<V> {
        let not = keyword_key_exists!(def, "not");

        if not.is_object() || not.is_bool() {
            Ok(Some(Box::new(validators::Not {
                url: helpers::alter_fragment_path(
                    ctx.url.clone(),
                    [ctx.escaped_fragment().as_ref(), "not"].join("/"),
                ),
            })))
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.join("/"),
                detail: "The value of `not` MUST be an object or a boolean".to_string(),
            })
        }
    }
}
