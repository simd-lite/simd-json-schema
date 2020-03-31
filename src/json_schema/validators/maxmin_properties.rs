use value_trait::*;

use super::error;
use super::scope;

#[allow(missing_copy_implementations)]
pub struct MaxProperties {
    pub length: u64,
}

impl<V> super::Validator<V> for MaxProperties
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
    fn validate(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState {
        let object = nonstrict_process!(val.as_object(), path);

        if (object.len() as u64) <= self.length {
            super::ValidationState::new()
        } else {
            val_error!(error::MaxProperties {
                path: path.to_string()
            })
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct MinProperties {
    pub length: u64,
}

impl<V> super::Validator<V> for MinProperties
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
    fn validate(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState {
        let object = nonstrict_process!(val.as_object(), path);

        if (object.len() as u64) >= self.length {
            super::ValidationState::new()
        } else {
            val_error!(error::MinProperties {
                path: path.to_string()
            })
        }
    }
}
