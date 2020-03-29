use std::any::{Any, TypeId};
use std::error::Error;
use std::fmt::Debug;

pub trait GetTypeId: Any {
    fn typeid(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

impl<T: Any> GetTypeId for T {}

pub fn get_data_ptr<T: ?Sized>(d: *const T) -> *const () {
    d as *const ()
}

pub trait SimdjsonSchemaError: Error + Send + Debug + GetTypeId {
    fn get_code(&self) -> &str;
    fn get_path(&self) -> &str;
    fn get_title(&self) -> &str;
    fn get_detail(&self) -> Option<&str> {
        None
    }
}

impl dyn SimdjsonSchemaError {
    pub fn is<E: SimdjsonSchemaError>(&self) -> bool {
        self.typeid() == TypeId::of::<E>()
    }

    pub fn downcast<E: SimdjsonSchemaError>(&self) -> Option<&E> {
        if self.is::<E>() {
            unsafe { Some(&*(get_data_ptr(self) as *const E)) }
        } else {
            None
        }
    }
}

pub type SimdjsonSchemaErrors = Vec<Box<dyn SimdjsonSchemaError>>;

macro_rules! impl_basic_err {
    ($err:ty, $code:expr) => {
        impl ::std::error::Error for $err {
            fn description(&self) -> &str {
                $code
            }
        }

        impl ::std::fmt::Display for $err {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                std::fmt::Display::fmt(&self.to_string(), formatter)
            }
        }
    };
}
macro_rules! impl_err {
    ($err:ty, $code:expr, $title:expr) => {
        impl_basic_err!($err, $code);

        impl SimdjsonSchemaError for $err {
            fn get_code(&self) -> &str {
                $code
            }
            fn get_title(&self) -> &str {
                $title
            }
            fn get_path(&self) -> &str {
                self.path.as_ref()
            }
        }
    };

    ($err:ty, $code:expr, $title:expr, +detail) => {
        impl_basic_err!($err, $code);

        impl SimdjsonSchemaError for $err {
            fn get_code(&self) -> &str {
                $code
            }
            fn get_title(&self) -> &str {
                $title
            }
            fn get_path(&self) -> &str {
                self.path.as_ref()
            }
            fn get_detail(&self) -> Option<&str> {
                Some(self.detail.as_ref())
            }
        }
    };

    ($err:ty, $code:expr, $title:expr, +opt_detail) => {
        impl_basic_err!($err, $code);

        impl SimdjsonSchemaError for $err {
            fn get_code(&self) -> &str {
                $code
            }
            fn get_title(&self) -> &str {
                $title
            }
            fn get_path(&self) -> &str {
                self.path.as_ref()
            }
            fn get_detail(&self) -> Option<&str> {
                self.detail.as_ref().map(|s| s.as_ref())
            }
        }
    };
}

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Properties {
    pub path: String,
    pub detail: String,
}
impl_err!(Properties, "properties", "Property conditions are not met", +detail);

#[derive(Debug)]
pub struct Required {
    pub path: String,
}
impl_err!(Required, "required", "This property is required");

#[derive(Debug)]
pub struct Format {
    pub path: String,
    pub detail: String,
}
impl_err!(Format, "format", "Format is wrong", +detail);

#[derive(Debug)]
pub struct Pattern {
    pub path: String,
}
impl_err!(Pattern, "pattern", "Pattern condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct WrongType {
    pub path: String,
    pub detail: String,
}
impl_err!(WrongType, "wrong_type", "Type of the value is wrong", +detail);

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct UniqueItems {
    pub path: String,
}
impl_err!(
    UniqueItems,
    "unique_items",
    "UniqueItems condition is not met"
);

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct AnyOf {
    pub path: String,
    pub states: Vec<super::validators::ValidationState>,
}
impl_err!(AnyOf, "any_of", "AnyOf conditions are not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct OneOf {
    pub path: String,
    pub states: Vec<super::validators::ValidationState>,
}
impl_err!(OneOf, "one_of", "OneOf conditions are not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MultipleOf {
    pub path: String,
}
impl_err!(MultipleOf, "multiple_of", "Wrong numer of the value");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Not {
    pub path: String,
}
impl_err!(Not, "not", "Not condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Maximum {
    pub path: String,
}
impl_err!(Maximum, "maximum", "Maximum condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Minimum {
    pub path: String,
}
impl_err!(Minimum, "minimum", "Minimum condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MaxLength {
    pub path: String,
}
impl_err!(MaxLength, "max_length", "MaxLength condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MinLength {
    pub path: String,
}
impl_err!(MinLength, "min_length", "MinLength condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MaxItems {
    pub path: String,
}
impl_err!(MaxItems, "max_items", "MaxItems condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MinItems {
    pub path: String,
}
impl_err!(MinItems, "min_items", "MinItems condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MaxProperties {
    pub path: String,
}
impl_err!(
    MaxProperties,
    "max_properties",
    "MaxProperties condition is not met"
);

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MinProperties {
    pub path: String,
}
impl_err!(
    MinProperties,
    "min_properties",
    "MinProperties condition is not met"
);

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Items {
    pub path: String,
    pub detail: String,
}
impl_err!(Items, "items", "Items condition is not met", +detail);

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Enum {
    pub path: String,
}
impl_err!(Enum, "enum", "Enum conditions are not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Contains {
    pub path: String,
}
impl_err!(Contains, "contains", "Contains condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Const {
    pub path: String,
}
impl_err!(Const, "const", "Const condition is not met");
