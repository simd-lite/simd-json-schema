use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

use super::error;
use super::scope;

#[allow(missing_copy_implementations)]
pub struct Enum {
    pub items: OwnedValue,
}

impl<V> super::Validator<V> for Enum
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
    fn validate(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState {
        let mut state = super::ValidationState::new();

        let mut contains = false;
        // FIXME: Ugly, dangerous and likely bad for null
        for value in self.items.as_array().unwrap() {
            if val.as_bool().is_some() && value.as_bool().is_some() {
                dbg!(val.as_bool().unwrap());
                dbg!(value.as_bool().unwrap());

                if val.as_bool().unwrap() == value.as_bool().unwrap() {
                    contains = true;
                    break;
                }
            } else if value.as_f64().is_some() && val.as_f64().is_some() {
                dbg!(value);

                if val.as_f64().unwrap() == value.as_f64().unwrap() {
                    contains = true;
                    break;
                }
            } else if value.as_i64().is_some() && val.as_i64().is_some() {
                dbg!(value);

                if val.as_i64().unwrap() == value.as_i64().unwrap() {
                    contains = true;
                    break;
                }
            } else if value.as_str().is_some() && val.as_str().is_some() {
                dbg!(val.as_str().unwrap());
                dbg!(value.as_str().unwrap());

                if val.as_str().unwrap() == value.as_str().unwrap() {
                    contains = true;
                    break;
                }
            }
        }

        if !contains {
            state.errors.push(Box::new(error::Enum {
                path: path.to_string(),
            }))
        }

        state
    }
}
