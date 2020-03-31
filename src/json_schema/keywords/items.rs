use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

use super::helpers;
use super::schema;
use super::validators;

#[allow(missing_copy_implementations)]
pub struct Items;
impl<V> super::Keyword<V> for Items
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
        let maybe_items = def.get("items");
        let maybe_additional = def.get("additionalItems");

        if !(maybe_items.is_some() || maybe_additional.is_some()) {
            return Ok(None);
        }

        let items = if maybe_items.is_some() {
            let items_val = maybe_items.unwrap();
            Some(if items_val.is_object() || items_val.is_bool() {
                validators::items::ItemsKind::Schema(helpers::alter_fragment_path(
                    ctx.url.clone(),
                    [ctx.escaped_fragment().as_ref(), "items"].join("/"),
                ))
            } else if items_val.is_array() {
                let mut schemas = vec![];
                for (idx, item) in items_val.as_array().unwrap().iter().enumerate() {
                    if item.is_object() || item.is_bool() {
                        schemas.push(helpers::alter_fragment_path(
                            ctx.url.clone(),
                            [
                                ctx.escaped_fragment().as_ref(),
                                "items",
                                idx.to_string().as_ref(),
                            ]
                            .join("/"),
                        ))
                    } else {
                        return Err(schema::SchemaError::Malformed {
                            path: ctx.fragment.join("/"),
                            detail: "Items of this array MUST be objects or booleans".to_string(),
                        });
                    }
                }

                validators::items::ItemsKind::Array(schemas)
            } else {
                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.join("/"),
                    detail: "`items` must be an object, an array or a boolean".to_string(),
                });
            })
        } else {
            None
        };

        let additional_items = if maybe_additional.is_some() {
            let additional_val = maybe_additional.unwrap();
            Some(if additional_val.is_bool() {
                validators::items::AdditionalKind::Boolean(additional_val.as_bool().unwrap())
            } else if additional_val.is_object() {
                validators::items::AdditionalKind::Schema(helpers::alter_fragment_path(
                    ctx.url.clone(),
                    [ctx.escaped_fragment().as_ref(), "additionalItems"].join("/"),
                ))
            } else {
                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.join("/"),
                    detail: "`additionalItems` must be a boolean or an object".to_string(),
                });
            })
        } else {
            None
        };

        Ok(Some(Box::new(validators::Items {
            items,
            additional: additional_items,
        })))
    }
}
