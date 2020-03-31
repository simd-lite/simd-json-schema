use value_trait::*;

use super::error;
use super::scope;

#[allow(missing_copy_implementations)]
pub struct MaxItems {
    pub length: u64,
}

impl<V> super::Validator<V> for MaxItems
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
        let array = nonstrict_process!(val.as_array(), path);

        if (array.len() as u64) <= self.length {
            super::ValidationState::new()
        } else {
            val_error!(error::MaxItems {
                path: path.to_string()
            })
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct MinItems {
    pub length: u64,
}

impl<V> super::Validator<V> for MinItems
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
        let array = nonstrict_process!(val.as_array(), path);

        if (array.len() as u64) >= self.length {
            super::ValidationState::new()
        } else {
            val_error!(error::MinItems {
                path: path.to_string()
            })
        }
    }
}
