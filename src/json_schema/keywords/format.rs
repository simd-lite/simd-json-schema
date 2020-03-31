use hashbrown::HashMap;
use value_trait::*;
use simd_json::value::owned::Value as OwnedValue;

use super::schema;
use super::validators;

pub type FormatBuilders<V> = HashMap<String, Box<dyn super::Keyword<V> + Send + Sync>>;

fn default_formats<V>() -> FormatBuilders<V>
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
    let mut map: FormatBuilders<V> = HashMap::new();

    let date_time_builder = Box::new(|_def: &OwnedValue, _ctx: &schema::WalkContext<'_>| {
        Ok(Some(
            Box::new(validators::formats::DateTime) as validators::BoxedValidator<V>
        ))
    });
    map.insert("date-time".to_string(), date_time_builder);

    let email_builder = Box::new(|_def: &OwnedValue, _ctx: &schema::WalkContext<'_>| {
        Ok(Some(
            Box::new(validators::formats::Email) as validators::BoxedValidator<V>
        ))
    });
    map.insert("email".to_string(), email_builder);

    let hostname_builder = Box::new(|_def: &OwnedValue, _ctx: &schema::WalkContext<'_>| {
        Ok(Some(
            Box::new(validators::formats::Hostname) as validators::BoxedValidator<V>
        ))
    });
    map.insert("hostname".to_string(), hostname_builder);

    let ipv4_builder = Box::new(|_def: &OwnedValue, _ctx: &schema::WalkContext<'_>| {
        Ok(Some(
            Box::new(validators::formats::Ipv4) as validators::BoxedValidator<V>
        ))
    });
    map.insert("ipv4".to_string(), ipv4_builder);

    let ipv6_builder = Box::new(|_def: &OwnedValue, _ctx: &schema::WalkContext<'_>| {
        Ok(Some(
            Box::new(validators::formats::Ipv6) as validators::BoxedValidator<V>
        ))
    });
    map.insert("ipv6".to_string(), ipv6_builder);

    let uri_builder = Box::new(|_def: &OwnedValue, _ctx: &schema::WalkContext<'_>| {
        Ok(Some(
            Box::new(validators::formats::Uri) as validators::BoxedValidator<V>
        ))
    });
    map.insert("uri".to_string(), uri_builder);

    let uri_reference_builder = Box::new(|_def: &OwnedValue, _ctx: &schema::WalkContext<'_>| {
        Ok(Some(
            Box::new(validators::formats::UriReference) as validators::BoxedValidator<V>
        ))
    });
    map.insert("uri-reference".to_string(), uri_reference_builder);

    let uuid_builder = Box::new(|_def: &OwnedValue, _ctx: &schema::WalkContext<'_>| {
        Ok(Some(
            Box::new(validators::formats::Uuid) as validators::BoxedValidator<V>
        ))
    });
    map.insert("uuid".to_string(), uuid_builder);

    map
}

pub struct Format<V> {
    pub formats: FormatBuilders<V>,
}

impl<V> Format<V>
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
    pub fn new() -> Format<V> {
        Format {
            formats: default_formats(),
        }
    }

    pub fn with<F>(build_formats: F) -> Format<V>
    where
        F: FnOnce(&mut FormatBuilders<V>),
    {
        let mut formats = default_formats();
        build_formats(&mut formats);
        Format { formats }
    }
}

impl<V> super::Keyword<V> for Format<V>
where
    V: Value
        + std::clone::Clone
        + std::convert::From<simd_json::value::owned::Value>
        + std::fmt::Display,
{
    fn compile(&self, def: &OwnedValue, ctx: &schema::WalkContext<'_>) -> super::KeywordCompilationResult<V>
    where
        <V as Value>::Key: std::borrow::Borrow<str> + std::hash::Hash + Eq,
    {
        let format = keyword_key_exists!(def, "format");

        if format.as_str().is_some() {
            let format = format.as_str().unwrap();
            match self.formats.get(format) {
                Some(keyword) => keyword.compile(def, ctx),
                None => Ok(None),
            }
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.join("/"),
                detail: "The value of format must be a string".to_string(),
            })
        }
    }
}
