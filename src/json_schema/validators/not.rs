use url;
use value_trait::*;

use super::error;
use super::scope;

#[allow(missing_copy_implementations)]
pub struct Not {
    pub url: url::Url,
}

impl<V> super::Validator<V> for Not
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
        let schema = scope.resolve(&self.url);
        let mut state = super::ValidationState::new();

        if schema.is_some() {
            if schema.unwrap().validate_in(val, path).is_valid() {
                state.errors.push(Box::new(error::Not {
                    path: path.to_string(),
                }))
            }
        } else {
            state.missing.push(self.url.clone());
        }

        state
    }
}
