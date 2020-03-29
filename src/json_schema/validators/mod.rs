use value_trait::*;
use super::scope;
use std::fmt;

pub use self::ref_::Ref;
pub mod ref_;

pub trait Validator<V>
where
    V: Value,
{
    fn validate(&self, item: &V, _: &str, _: &scope::Scope<V>) -> ValidationState
    where
        <V as Value>::Key:
            std::borrow::Borrow<str> + std::hash::Hash + Eq + std::convert::AsRef<str>;
}

#[derive(Debug)]
pub struct ValidationState {
    pub errors: super::error::SimdjsonSchemaErrors,
    pub missing: Vec<url::Url>,
}

impl ValidationState {
    pub fn new() -> ValidationState {
        ValidationState {
            errors: vec![],
            missing: vec![],
        }
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn append(&mut self, second: ValidationState) {
        self.errors.extend(second.errors);
        self.missing.extend(second.missing);
    }
}

impl<V> fmt::Debug for dyn Validator<V> + Send + Sync
where
    V: Value,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("<validator>")
    }
}

pub type BoxedValidator<V> = Box<dyn Validator<V> + Send + Sync>;
pub type Validators<V> = Vec<BoxedValidator<V>>;
