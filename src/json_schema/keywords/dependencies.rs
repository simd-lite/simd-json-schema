use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

use super::super::helpers;
use super::super::schema;
use super::super::validators;

#[allow(missing_copy_implementations)]
pub struct Dependencies;
impl<V> super::Keyword<V> for Dependencies
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
        let deps = keyword_key_exists!(def, "dependencies");

        if !deps.is_object() {
            return Err(schema::SchemaError::Malformed {
                path: ctx.fragment.join("/"),
                detail: "The value of this keyword must be an object.".to_string(),
            });
        }

        let deps = deps.as_object().unwrap();
        let mut items = hashbrown::HashMap::new();

        for (key, item) in deps.iter() {
            if item.is_object() || item.is_bool() {
                items.insert(
                    key.clone().to_string(),
                    validators::dependencies::DepKind::Schema(helpers::alter_fragment_path(
                        ctx.url.clone(),
                        [
                            ctx.escaped_fragment().as_ref(),
                            "dependencies",
                            helpers::encode(key.as_ref()).as_ref(),
                        ]
                        .join("/"),
                    )),
                );
            } else if item.is_array() {
                let item = item.as_array().unwrap();
                let mut keys = vec![];
                for key in item.iter() {
                    if key.is_str() {
                        keys.push(key.as_str().unwrap().to_string())
                    } else {
                        return Err(schema::SchemaError::Malformed {
                            path: ctx.fragment.join("/"),
                            detail: "Each element must be a string, and elements in the array must be unique.".to_string()
                        });
                    }
                }
                items.insert(
                    key.clone().to_string(),
                    validators::dependencies::DepKind::Property(keys),
                );
            } else {
                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.join("/"),
                    detail:
                        "Each value of this object must be either an object, an array or a boolean."
                            .to_string(),
                });
            }
        }

        Ok(Some(Box::new(validators::Dependencies { items })))
    }
}
