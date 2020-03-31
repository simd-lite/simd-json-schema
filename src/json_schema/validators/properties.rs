use regex;
use url;
use value_trait::*;

use super::super::scope;
use super::error;

#[derive(Debug)]
pub enum AdditionalKind {
    Boolean(bool),
    Schema(url::Url),
}

#[allow(missing_copy_implementations)]
pub struct Properties {
    pub properties: hashbrown::HashMap<String, url::Url>,
    pub additional: AdditionalKind,
    pub patterns: Vec<(regex::Regex, url::Url)>,
}

impl<V> super::Validator<V> for Properties
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
    fn validate(&self, val: &V, path: &str, scope: &scope::Scope<V>) -> super::ValidationState {
        let object = nonstrict_process!(val.as_object(), path);
        let mut state = super::ValidationState::new();

        'main: for (key, value) in object.iter() {
            let is_property_passed = if self.properties.contains_key(key.as_ref()) {
                let url = &self.properties[key.as_ref()];
                let schema = scope.resolve(url);
                if schema.is_some() {
                    let value_path = [path, key.as_ref()].join("/");
                    state.append(schema.unwrap().validate_in(value, value_path.as_ref()))
                } else {
                    state.missing.push(url.clone())
                }

                true
            } else {
                false
            };

            let mut is_pattern_passed = false;
            for &(ref regex, ref url) in self.patterns.iter() {
                if regex.is_match(key.as_ref()) {
                    let schema = scope.resolve(url);
                    if schema.is_some() {
                        let value_path = [path, key.as_ref()].join("/");
                        state.append(schema.unwrap().validate_in(value, value_path.as_ref()));
                        is_pattern_passed = true;
                    } else {
                        state.missing.push(url.clone())
                    }
                }
            }

            if is_property_passed || is_pattern_passed {
                continue 'main;
            }

            match self.additional {
                AdditionalKind::Boolean(allowed) if !allowed => {
                    state.errors.push(Box::new(error::Properties {
                        path: path.to_string(),
                        detail: "Additional properties are not allowed".to_string(),
                    }))
                }
                AdditionalKind::Schema(ref url) => {
                    let schema = scope.resolve(url);

                    if schema.is_some() {
                        let value_path = [path, key.as_ref()].join("/");
                        state.append(schema.unwrap().validate_in(value, value_path.as_ref()))
                    } else {
                        state.missing.push(url.clone())
                    }
                }
                // Additional are allowed here
                _ => (),
            }
        }

        state
    }
}
