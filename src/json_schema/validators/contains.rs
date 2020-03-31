use url;
use value_trait::*;

use super::error;
use super::scope;

#[allow(missing_copy_implementations)]
pub struct Contains {
    pub url: url::Url,
}

impl<V> super::Validator<V> for Contains
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
        let array = nonstrict_process!(val.as_array(), path);

        let schema = scope.resolve(&self.url);
        let mut state = super::ValidationState::new();

        if schema.is_some() {
            let schema = schema.unwrap();
            let any_matched = array.iter().enumerate().any(|(idx, item)| {
                let item_path = [path, idx.to_string().as_ref()].join("/");
                schema.validate_in(item, item_path.as_ref()).is_valid()
            });

            if !any_matched {
                state.errors.push(Box::new(error::Contains {
                    path: path.to_string(),
                }))
            }
        } else {
            state.missing.push(self.url.clone());
        }

        state
    }
}
