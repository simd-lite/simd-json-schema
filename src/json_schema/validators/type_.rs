use value_trait::*;

use super::error;
use super::primitive_types as pt;
use super::scope;

#[derive(Debug)]
pub enum TypeKind {
    Single(pt::PrimitiveType),
    Set(Vec<pt::PrimitiveType>),
}

#[allow(missing_copy_implementations)]
pub struct Type {
    pub item: TypeKind,
}

fn check_type<V: Value>(val: &V, ty: pt::PrimitiveType) -> bool {
    match ty {
        pt::PrimitiveType::Array => val.is_array(),
        pt::PrimitiveType::Boolean => val.is_bool(),
        pt::PrimitiveType::Integer => {
            let is_true_integer = val.is_u64() || val.is_i64();
            let is_integer_float = val.is_f64() && val.as_f64().unwrap().fract() == 0.0;
            is_true_integer || is_integer_float
        }
        pt::PrimitiveType::Number => (val.is_f64() || val.is_i64() || val.is_u64()),
        pt::PrimitiveType::Null => val.is_null(),
        pt::PrimitiveType::Object => val.is_object(),
        pt::PrimitiveType::String => val.is_str(),
    }
}

impl<V> super::Validator<V> for Type
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
        let mut state = super::ValidationState::new();

        match self.item {
            TypeKind::Single(t) => {
                if !check_type(val, t) {
                    state.errors.push(Box::new(error::WrongType {
                        path: path.to_string(),
                        detail: format!("The value must be {}", t),
                    }))
                }
            }
            TypeKind::Set(ref set) => {
                let mut is_type_match = false;
                for ty in set.iter() {
                    if check_type(val, *ty) {
                        is_type_match = true;
                        break;
                    }
                }

                if !is_type_match {
                    state.errors.push(Box::new(error::WrongType {
                        path: path.to_string(),
                        detail: format!(
                            "The value must be any of: {}",
                            set.iter()
                                .map(|ty| ty.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        ),
                    }))
                }
            }
        }

        state
    }
}
