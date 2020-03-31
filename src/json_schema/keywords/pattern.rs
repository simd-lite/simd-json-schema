use regex;
use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

use super::schema;
use super::validators;

#[allow(missing_copy_implementations)]
pub struct Pattern;
impl<V: std::string::ToString> super::Keyword<V> for Pattern
where
    V: Value + std::clone::Clone + std::convert::From<simd_json::value::owned::Value>,
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
        let pattern = keyword_key_exists!(def, "pattern");

        if pattern.is_str() {
            let pattern_val = pattern.as_str().unwrap();
            match regex::Regex::new(pattern_val) {
                Ok(re) => Ok(Some(Box::new(validators::Pattern { regex: re }))),
                Err(err) => Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.join("/"),
                    detail: format!(
                        "The value of pattern must be a valid regular expression, but {:?}",
                        err
                    ),
                }),
            }
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.join("/"),
                detail: "The value of pattern must be a string".to_string(),
            })
        }
    }
}
