use url;
use value_trait::*;

use super::error;
use super::scope;

#[derive(Debug)]
pub enum DepKind {
    Schema(url::Url),
    Property(Vec<String>),
}

#[allow(missing_copy_implementations)]
pub struct Dependencies {
    pub items: hashbrown::HashMap<String, DepKind>,
}

impl<V> super::Validator<V> for Dependencies
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
    fn validate(&self, object: &V, path: &str, scope: &scope::Scope<V>) -> super::ValidationState {
        if !object.is_object() {
            return super::ValidationState::new();
        }

        let mut state = super::ValidationState::new();

        for (key, dep) in self.items.iter() {
            if object.get(&key).is_some() {
                match dep {
                    DepKind::Schema(ref url) => {
                        let schema = scope.resolve(url);
                        if schema.is_some() {
                            state.append(schema.unwrap().validate_in(object, path));
                        } else {
                            state.missing.push(url.clone())
                        }
                    }
                    DepKind::Property(ref keys) => {
                        for key in keys.iter() {
                            if object.get(&key).is_none() {
                                state.errors.push(Box::new(error::Required {
                                    path: [path, key.as_ref()].join("/"),
                                }))
                            }
                        }
                    }
                }
            }
        }

        state
    }
}
