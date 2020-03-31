use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

use super::schema;
use super::validators;

macro_rules! kw_minmax_integer {
    ($name:ident, $keyword:expr) => {
        #[allow(missing_copy_implementations)]
        pub struct $name;
        impl<V> super::Keyword<V> for $name
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
            fn compile(
                &self,
                def: &OwnedValue,
                ctx: &schema::WalkContext<'_>,
            ) -> super::KeywordCompilationResult<V> {
                let length = keyword_key_exists!(def, $keyword);

                if length.is_f64() {
                    let length_val = length.as_f64().unwrap();
                    if length_val >= 0f64 && length_val.fract() == 0f64 {
                        Ok(Some(Box::new(validators::$name {
                            length: length_val as u64,
                        })))
                    } else {
                        Err(schema::SchemaError::Malformed {
                            path: ctx.fragment.join("/"),
                            detail: "The value must be a positive integer or zero".to_string(),
                        })
                    }
                } else {
                    Err(schema::SchemaError::Malformed {
                        path: ctx.fragment.join("/"),
                        detail: "The value must be a positive integer or zero".to_string(),
                    })
                }
            }
        }
    };
}

kw_minmax_integer!(MaxLength, "maxLength");
kw_minmax_integer!(MinLength, "minLength");
