use regex;
use value_trait::*;

use super::error;
use super::scope;

#[allow(missing_copy_implementations)]
pub struct Pattern {
    pub regex: regex::Regex,
}

impl<V> super::Validator<V> for Pattern
where
    V: Value,
{
    fn validate(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        if self.regex.is_match(string) {
            super::ValidationState::new()
        } else {
            val_error!(error::Pattern {
                path: path.to_string()
            })
        }
    }
}
