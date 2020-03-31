use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

use super::helpers;
use super::schema;
use super::validators;

macro_rules! of_keyword {
    ($name:ident, $kw:expr) => {
        #[allow(missing_copy_implementations)]
        pub struct $name;
        impl<V> super::Keyword<V> for $name
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
                let of = keyword_key_exists!(def, $kw);

                if of.is_array() {
                    let of = of.as_array().unwrap();

                    if of.len() == 0 {
                        return Err(schema::SchemaError::Malformed {
                            path: ctx.fragment.join("/"),
                            detail: "This array MUST have at least one element.".to_string(),
                        });
                    }

                    let mut schemes = vec![];
                    for (idx, scheme) in of.iter().enumerate() {
                        if scheme.is_object() || scheme.is_bool() {
                            schemes.push(helpers::alter_fragment_path(
                                ctx.url.clone(),
                                [
                                    ctx.escaped_fragment().as_ref(),
                                    $kw,
                                    idx.to_string().as_ref(),
                                ]
                                .join("/"),
                            ))
                        } else {
                            return Err(schema::SchemaError::Malformed {
                                path: ctx.fragment.join("/"),
                                detail: "Elements of the array MUST be objects or booleans."
                                    .to_string(),
                            });
                        }
                    }

                    Ok(Some(Box::new(validators::$name { schemes: schemes })))
                } else {
                    Err(schema::SchemaError::Malformed {
                        path: ctx.fragment.join("/"),
                        detail: "The value of this keyword MUST be an array.".to_string(),
                    })
                }
            }
        }
    };
}

of_keyword!(AllOf, "allOf");
of_keyword!(AnyOf, "anyOf");
of_keyword!(OneOf, "oneOf");
