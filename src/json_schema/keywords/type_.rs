use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

use super::schema;
use super::validators;

#[allow(missing_copy_implementations)]
pub struct Type;
impl<V: std::string::ToString> super::Keyword<V> for Type
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
        let type_ = keyword_key_exists!(def, "type");

        if type_.is_str() {
            let ty = type_.as_str().unwrap().parse().ok();

            if ty.is_some() {
                Ok(Some(Box::new(validators::Type {
                    item: validators::type_::TypeKind::Single(ty.unwrap()),
                })))
            } else {
                Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.join("/"),
                    detail: format!(
                        "String values must be one of the seven primitive types defined by the core specification. Unknown type: {}",
                        type_.as_str().unwrap()
                    )
                })
            }
        } else if type_.is_array() {
            let types = type_.as_array().unwrap();

            if types.is_empty() {
                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.join("/"),
                    detail: "This array must have at least one element.".to_string(),
                });
            }

            let mut converted_types = vec![];
            for ty in types.iter() {
                if ty.is_str() {
                    let converted_ty = ty.as_str().unwrap().parse().ok();
                    if converted_ty.is_some() {
                        converted_types.push(converted_ty.unwrap());
                    } else {
                        return Err(schema::SchemaError::Malformed {
                            path: ctx.fragment.join("/"),
                            detail: format!("Unknown type: {}", ty.as_str().unwrap()),
                        });
                    }
                } else {
                    return Err(schema::SchemaError::Malformed {
                        path: ctx.fragment.join("/"),
                        detail: "String values must be one of the seven primitive types defined by the core specification.".to_string()
                    });
                }
            }

            Ok(Some(Box::new(validators::Type {
                item: validators::type_::TypeKind::Set(converted_types),
            })))
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.join("/"),
                detail: "The value of this keyword must be either a string or an array."
                    .to_string(),
            })
        }
    }
}
