use simd_json::json;
use url::Url;
use uuid::Uuid;
use value_trait::*;

use super::schema;

pub fn encode(string: &str) -> String {
    percent_encoding::percent_encode(
        string
            .replace("~", "~0")
            .replace("/", "~1")
            .replace("%", "%25")
            .as_bytes(),
        percent_encoding::NON_ALPHANUMERIC,
    )
    .to_string()
}

pub fn connect(strings: &[&str]) -> String {
    strings
        .iter()
        .map(|s| encode(s))
        .collect::<Vec<String>>()
        .join("/")
}

pub fn generate_id() -> Url {
    let uuid = Uuid::new_v4();
    Url::parse(&format!("json-schema://{}", uuid)).unwrap()
}

pub fn parse_url_key<V: Value>(key: &str, obj: &V) -> Result<Option<Url>, schema::SchemaError>
where
    V: Value,
    <V as Value>::Key: std::borrow::Borrow<str> + std::hash::Hash + Eq + std::convert::AsRef<str>,
{
    match obj.get(key) {
        Some(value) => match value.as_str() {
            Some(string) => Url::parse(string)
                .map(Some)
                .map_err(schema::SchemaError::UrlParseError),
            None => Ok(None),
        },
        None => Ok(None),
    }
}

pub fn alter_fragment_path(mut url: Url, new_fragment: String) -> Url {
    let normalized_fragment = if new_fragment.starts_with('/') {
        &new_fragment[1..]
    } else {
        new_fragment.as_ref()
    };

    let result_fragment = match url.fragment() {
        Some(ref fragment) if !fragment.is_empty() => {
            if !fragment.starts_with('/') {
                let mut result_fragment = "".to_string();
                let mut fragment_parts = fragment.split('/').map(|s| s.to_string());
                result_fragment.push_str("#");
                result_fragment.push_str(fragment_parts.next().unwrap().as_ref());
                result_fragment.push_str("/");
                result_fragment.push_str(normalized_fragment.as_ref());
                result_fragment
            } else {
                "/".to_string() + normalized_fragment
            }
        }
        _ => "/".to_string() + normalized_fragment,
    };

    url.set_fragment(Some(&result_fragment));
    url
}

pub fn serialize_schema_path(url: &Url) -> (String, Option<String>) {
    let mut url_without_fragment = url.clone();
    url_without_fragment.set_fragment(None);
    let mut url_str = url_without_fragment.into_string();

    match url.fragment().as_ref() {
        Some(fragment) if !fragment.is_empty() => {
            if !fragment.starts_with('/') {
                let fragment_parts = fragment
                    .split('/')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                url_str.push_str("#");
                url_str.push_str(fragment_parts[0].as_ref());
                let fragment = if fragment_parts.len() > 1 {
                    Some("/".to_string() + fragment_parts[1..].join("/").as_ref())
                } else {
                    None
                };
                (url_str, fragment)
            } else {
                (url_str, Some(fragment.to_string()))
            }
        }
        _ => (url_str, None),
    }
}

pub fn convert_boolean_schema<V: Value>(val: V) -> V
where
    V: Value + std::convert::From<simd_json::value::owned::Value>,
    <V as Value>::Key: std::borrow::Borrow<str> + std::hash::Hash + Eq + std::convert::AsRef<str>,
{
    match val.as_bool() {
        Some(b) => {
            if b {
                json!({}).into()
            } else {
                json!({"not": {}}).into()
            }
        }
        None => val,
    }
}

pub fn parse_url_key_with_base<V: Value>(
    key: &str,
    obj: &V,
    base: &Url,
) -> Result<Option<Url>, schema::SchemaError>
where
    <V as Value>::Key: std::borrow::Borrow<str> + std::hash::Hash + Eq + std::convert::AsRef<str>,
{
    match obj.get(key) {
        Some(value) => match value.as_str() {
            Some(string) => Url::options()
                .base_url(Some(base))
                .parse(string)
                .map(Some)
                .map_err(schema::SchemaError::UrlParseError),
            None => Ok(None),
        },
        None => Ok(None),
    }
}
