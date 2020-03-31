use value_trait::*;

use super::error;
use super::scope;

#[allow(missing_copy_implementations)]
pub struct Required {
    pub items: Vec<String>,
}

impl<V> super::Validator<V> for Required
where
    V: Value,
{
    fn validate(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState
    where
        <V as Value>::Key: std::borrow::Borrow<str> + std::hash::Hash + Eq,
    {
        let mut state = super::ValidationState::new();

        for key in self.items.iter() {
            if val.get(key.as_str()).is_none() {
                state.errors.push(Box::new(error::Required {
                    path: [path, key.as_ref()].join("/"),
                }))
            }
        }

        state
    }
}
