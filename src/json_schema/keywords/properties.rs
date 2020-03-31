use regex;
use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

use super::super::helpers;
use super::schema;
use super::validators;

#[allow(missing_copy_implementations)]
pub struct Properties;
impl<V> super::Keyword<V> for Properties
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
        let maybe_properties = def.get("properties");
        let maybe_additional = def.get("additionalProperties");
        let maybe_pattern = def.get("patternProperties");

        if maybe_properties.is_none() && maybe_additional.is_none() && maybe_pattern.is_none() {
            return Ok(None);
        }

        let properties = if maybe_properties.is_some() {
            let properties = maybe_properties.unwrap();
            if properties.is_object() {
                let mut schemes = hashbrown::HashMap::new();
                let properties = properties.as_object().unwrap();
                for (key, value) in properties.iter() {
                    if value.is_object() || value.is_bool() {
                        schemes.insert(
                            key.to_string(),
                            helpers::alter_fragment_path(
                                ctx.url.clone(),
                                [
                                    ctx.escaped_fragment().as_ref(),
                                    "properties",
                                    helpers::encode(key.as_ref()).as_ref(),
                                ]
                                .join("/"),
                            ),
                        );
                    } else {
                        return Err(schema::SchemaError::Malformed {
                            path: ctx.fragment.join("/"),
                            detail: "Each value of this object must be an object or a boolean"
                                .to_string(),
                        });
                    }
                }
                schemes
            } else {
                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.join("/"),
                    detail: "The value of `properties` must be an object.".to_string(),
                });
            }
        } else {
            hashbrown::HashMap::new()
        };

        let additional_properties = if maybe_additional.is_some() {
            let additional_val = maybe_additional.unwrap();
            if additional_val.is_bool() {
                validators::properties::AdditionalKind::Boolean(additional_val.as_bool().unwrap())
            } else if additional_val.is_object() {
                validators::properties::AdditionalKind::Schema(helpers::alter_fragment_path(
                    ctx.url.clone(),
                    [ctx.escaped_fragment().as_ref(), "additionalProperties"].join("/"),
                ))
            } else {
                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.join("/"),
                    detail: "The value of `additionalProperties` must be a boolean or an object."
                        .to_string(),
                });
            }
        } else {
            validators::properties::AdditionalKind::Boolean(true)
        };

        let patterns = if maybe_pattern.is_some() {
            let pattern = maybe_pattern.unwrap();
            if pattern.is_object() {
                let pattern = pattern.as_object().unwrap();
                let mut patterns = vec![];

                for (key, value) in pattern.iter() {
                    if value.is_object() || value.is_bool() {
                        match regex::Regex::new(key.as_ref()) {
                            Ok(regex) => {
                                let url = helpers::alter_fragment_path(ctx.url.clone(), [
                                    ctx.escaped_fragment().as_ref(),
                                    "patternProperties",
                                    helpers::encode(key.as_ref()).as_ref()
                                ].join("/"));
                                patterns.push((regex, url));
                            },
                            Err(_) => {
                                return Err(schema::SchemaError::Malformed {
                                    path: ctx.fragment.join("/"),
                                    detail: "Each property name of this object SHOULD be a valid regular expression.".to_string()
                                })
                            }
                        }
                    } else {
                        return Err(schema::SchemaError::Malformed {
                            path: ctx.fragment.join("/"),
                            detail: "Each value of this object must be an object or a boolean"
                                .to_string(),
                        });
                    }
                }

                patterns
            } else {
                return Err(schema::SchemaError::Malformed {
                    path: ctx.fragment.join("/"),
                    detail: "The value of `patternProperties` must be an object".to_string(),
                });
            }
        } else {
            vec![]
        };

        Ok(Some(Box::new(validators::Properties {
            properties,
            additional: additional_properties,
            patterns,
        })))
    }
}
