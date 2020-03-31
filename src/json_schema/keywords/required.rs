use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

use super::schema;
use super::validators;

#[allow(missing_copy_implementations)]
pub struct Required;
impl<V> super::Keyword<V> for Required
where
    V: Value
        + std::clone::Clone
        + std::convert::From<simd_json::value::owned::Value>
        + std::string::ToString,
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
        let required = keyword_key_exists!(def, "required");

        if required.is_array() {
            let required = required.as_array().unwrap();

            let mut items = vec![];
            for item in required.iter() {
                if item.is_str() {
                    items.push(item.to_string());
                } else {
                    return Err(schema::SchemaError::Malformed {
                        path: ctx.fragment.join("/"),
                        detail: "The values of `required` must be string".to_string(),
                    });
                }
            }

            Ok(Some(Box::new(validators::Required { items })))
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.join("/"),
                detail: "The value of this keyword must be an array.".to_string(),
            })
        }
    }
}
