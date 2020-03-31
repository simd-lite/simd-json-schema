use url;
use value_trait::*;

use super::error;
use super::scope;

#[allow(missing_copy_implementations)]
pub struct AllOf {
    pub schemes: Vec<url::Url>,
}

impl<V> super::Validator<V> for AllOf
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
        let mut state = super::ValidationState::new();

        for url in self.schemes.iter() {
            let schema = scope.resolve(url);

            if schema.is_some() {
                state.append(schema.unwrap().validate_in(val, path))
            } else {
                state.missing.push(url.clone())
            }
        }

        state
    }
}

#[allow(missing_copy_implementations)]
pub struct AnyOf {
    pub schemes: Vec<url::Url>,
}

impl<V> super::Validator<V> for AnyOf
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
        let mut state = super::ValidationState::new();

        let mut states = vec![];
        let mut valid = false;
        for url in self.schemes.iter() {
            let schema = scope.resolve(url);

            if schema.is_some() {
                let current_state = schema.unwrap().validate_in(val, path);

                state.missing.extend(current_state.missing.clone());

                if current_state.is_valid() {
                    valid = true;
                    break;
                } else {
                    states.push(current_state)
                }
            } else {
                state.missing.push(url.clone())
            }
        }

        if !valid {
            state.errors.push(Box::new(error::AnyOf {
                path: path.to_string(),
                states,
            }))
        }

        state
    }
}

#[allow(missing_copy_implementations)]
pub struct OneOf {
    pub schemes: Vec<url::Url>,
}

impl<V> super::Validator<V> for OneOf
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
        let mut state = super::ValidationState::new();

        let mut states = vec![];
        let mut valid = 0;
        for url in self.schemes.iter() {
            let schema = scope.resolve(url);

            if schema.is_some() {
                let current_state = schema.unwrap().validate_in(val, path);

                state.missing.extend(current_state.missing.clone());

                if current_state.is_valid() {
                    valid += 1;
                } else {
                    states.push(current_state)
                }
            } else {
                state.missing.push(url.clone())
            }
        }

        if valid != 1 {
            state.errors.push(Box::new(error::OneOf {
                path: path.to_string(),
                states,
            }))
        }

        state
    }
}
