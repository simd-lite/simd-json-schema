use value_trait::*;

use super::error;
use super::scope;
use std::cmp::Ordering;
use std::f64;

#[allow(missing_copy_implementations)]
pub struct MultipleOf {
    pub number: f64,
}

impl<V> super::Validator<V> for MultipleOf
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
    // NOTE: User must pass in a value that can be represented as f64. 3.0 will work but not 3.
    fn validate(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState {
        let number = strict_process!(
            val.as_f64(),
            path,
            "Number must end with decimal to be compared as multiple of"
        );

        let valid = if (number.fract() == 0f64) && (self.number.fract() == 0f64) {
            (number % self.number) == 0f64
        } else {
            let remainder: f64 = (number / self.number) % 1f64;
            let remainder_less_than_epsilon = match remainder.partial_cmp(&f64::EPSILON) {
                None | Some(Ordering::Less) => true,
                _ => false,
            };
            let remainder_less_than_one = remainder < (1f64 - f64::EPSILON);
            remainder_less_than_epsilon && remainder_less_than_one
        };

        if valid {
            super::ValidationState::new()
        } else {
            val_error!(error::MultipleOf {
                path: path.to_string()
            })
        }
    }
}
