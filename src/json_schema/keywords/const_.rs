use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

use super::super::schema;
use super::super::validators;

pub struct Const;
impl<V> super::Keyword<V> for Const
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
    fn compile(
        &self,
        def: &OwnedValue,
        _ctx: &schema::WalkContext<'_>,
    ) -> super::KeywordCompilationResult<V> {
        let const_ = keyword_key_exists!(def, "const");

        Ok(Some(Box::new(validators::Const {
            item: const_.clone(),
        })))
    }
}
