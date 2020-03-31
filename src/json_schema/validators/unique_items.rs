use value_trait::*;

use super::error;
use super::scope;

#[allow(missing_copy_implementations)]
pub struct UniqueItems;
impl<V> super::Validator<V> for UniqueItems
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

        let mut unique = true;
        'main: for (idx, item_i) in array.iter().enumerate() {
            for item_j in array.get(..idx).unwrap().iter() {
                if item_i.as_str() == item_j.as_str() {
                    unique = false;
                    break 'main;
                }
            }

            for item_j in array.get((idx + 1)..).unwrap().iter() {
                if item_i.as_str() == item_j.as_str() {
                    unique = false;
                    break 'main;
                }
            }
        }

        if unique {
            super::ValidationState::new()
        } else {
            val_error!(error::UniqueItems {
                path: path.to_string()
            })
        }
    }
}
