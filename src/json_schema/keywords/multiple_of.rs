use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

use super::super::schema;
use super::super::validators;

#[allow(missing_copy_implementations)]
pub struct MultipleOf;
impl<V: 'static> super::Keyword<V> for MultipleOf
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
        let multiple_of = keyword_key_exists!(def, "multipleOf");

        if multiple_of.is_f64() {
            let multiple_of = multiple_of.as_f64().unwrap();
            if multiple_of > 0f64 {
                Ok(Some(Box::new(validators::MultipleOf {
                    number: multiple_of,
                })))
            } else {
                Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.join("/"),
                    detail: "The value of multipleOf must be strictly greater than 0".to_string(),
                })
            }
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.join("/"),
                detail: "The value of multipleOf must be a JSON number".to_string(),
            })
        }
    }
}
