use value_trait::*;

use super::scope;

pub struct Ref {
    pub url: url::Url,
}

impl<V> super::Validator<V> for Ref
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

        if schema.is_some() {
            schema.unwrap().validate_in(val, path)
        } else {
            let mut state = super::ValidationState::new();
            state.missing.push(self.url.clone());
            state
        }
    }
}
