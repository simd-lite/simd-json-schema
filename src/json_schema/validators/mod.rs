use super::error;
use super::primitive_types;
use super::scope;
use std::fmt;
use value_trait::*;

macro_rules! nonstrict_process {
    ($val:expr, $path:ident) => {{
        let maybe_val = $val;
        if maybe_val.is_none() {
            return $crate::json_schema::validators::ValidationState::new();
        }

        maybe_val.unwrap()
    }};
}

macro_rules! val_error {
    ($err:expr) => {
        $crate::json_schema::validators::ValidationState {
            errors: vec![Box::new($err)],
            missing: vec![],
        }
    };
}

#[macro_export]
macro_rules! strict_process {
    ($val:expr, $path:ident, $err:expr) => {{
        let maybe_val = $val;
        if maybe_val.is_none() {
            return val_error!($crate::json_schema::error::WrongType {
                path: $path.to_string(),
                detail: $err.to_string()
            });
        }

        maybe_val.unwrap()
    }};
}

pub use self::const_::Const;
pub use self::contains::Contains;
pub use self::dependencies::Dependencies;
pub use self::enum_::Enum;
pub use self::items::Items;
pub use self::maxmin::{ExclusiveMaximum, ExclusiveMinimum, Maximum, Minimum};
pub use self::maxmin_items::{MaxItems, MinItems};
pub use self::maxmin_length::{MaxLength, MinLength};
pub use self::maxmin_properties::{MaxProperties, MinProperties};
pub use self::multiple_of::MultipleOf;
pub use self::not::Not;
pub use self::of::{AllOf, AnyOf, OneOf};
pub use self::pattern::Pattern;
pub use self::properties::Properties;
pub use self::property_names::PropertyNames;
pub use self::ref_::Ref;
pub use self::required::Required;
pub use self::type_::Type;
pub use self::unique_items::UniqueItems;

pub mod const_;
pub mod contains;
pub mod dependencies;
pub mod enum_;
pub mod items;
//pub mod formats;
mod maxmin;
mod maxmin_items;
mod maxmin_length;
mod maxmin_properties;
pub mod multiple_of;
pub mod not;
pub mod of;
pub mod pattern;
pub mod properties;
pub mod property_names;
pub mod ref_;
pub mod required;
pub mod type_;
pub mod unique_items;

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
