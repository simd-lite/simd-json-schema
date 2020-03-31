use value_trait::*;

use super::error;
use super::scope;

#[allow(missing_copy_implementations)]
pub struct Maximum {
    pub number: f64,
}

impl<V> super::Validator<V> for Maximum
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
        let number = nonstrict_process!(val.as_f64(), path);

        if number <= self.number {
            super::ValidationState::new()
        } else {
            val_error!(error::Maximum {
                path: path.to_string()
            })
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct ExclusiveMaximum {
    pub number: f64,
}

impl<V> super::Validator<V> for ExclusiveMaximum
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
        let number = nonstrict_process!(val.as_f64(), path);

        if number < self.number {
            super::ValidationState::new()
        } else {
            val_error!(error::Maximum {
                path: path.to_string()
            })
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Minimum {
    pub number: f64,
}

impl<V> super::Validator<V> for Minimum
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
        let number = nonstrict_process!(val.as_f64(), path);

        if number >= self.number {
            super::ValidationState::new()
        } else {
            val_error!(error::Minimum {
                path: path.to_string()
            })
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct ExclusiveMinimum {
    pub number: f64,
}

impl<V> super::Validator<V> for ExclusiveMinimum
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
        let number = nonstrict_process!(val.as_f64(), path);

        if number > self.number {
            super::ValidationState::new()
        } else {
            val_error!(error::Minimum {
                path: path.to_string()
            })
        }
    }
}
