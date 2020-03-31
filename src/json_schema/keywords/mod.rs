use simd_json::value::owned::Value as OwnedValue;
use std::any;
use std::fmt;
use std::sync::Arc;
use value_trait::Value;

use super::helpers;
use super::schema;
use super::validators;

pub type KeywordPair<V> = (Vec<String>, Box<dyn Keyword<V>>);
pub type KeywordMap<V> = hashbrown::HashMap<String, Arc<KeywordConsumer<V>>>;
pub type KeywordCompilationResult<V> =
    Result<Option<validators::BoxedValidator<V>>, schema::SchemaError>;

pub trait Keyword<V>: Send + Sync + any::Any {
    fn compile(&self, src: &OwnedValue, ctx: &schema::WalkContext) -> KeywordCompilationResult<V>;
    fn is_exclusive(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct KeywordConsumer<V>
where
    V: Value,
{
    pub keys: Vec<String>,
    pub keyword: Box<dyn Keyword<V>>,
}

impl<V> KeywordConsumer<V>
where
    V: Value,
{
    pub fn consume(&self, set: &mut hashbrown::HashSet<String>) {
        for key in self.keys.iter() {
            if set.contains(key) {
                set.remove(key);
            }
        }
    }
}

impl<V> fmt::Debug for dyn Keyword<V> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("<keyword>")
    }
}

macro_rules! keyword_key_exists {
    ($val:expr, $key:expr) => {{
        let maybe_val = $val.get($key);

        if maybe_val.is_none() {
            return Ok(None);
        } else {
            maybe_val.unwrap()
        }
    }};
}

pub mod const_;
pub mod contains;
pub mod dependencies;
pub mod enum_;
// FIXME: Reimplement without value trait for keyword val
//pub mod format;
pub mod items;
#[macro_use]
pub mod maxmin_length;
pub mod maxmin;
pub mod maxmin_items;
pub mod maxmin_properties;
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

pub fn default<'scope, V: 'scope>() -> KeywordMap<V>
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
    let mut map = hashbrown::HashMap::new();

    decouple_keyword((vec!["allOf".to_string()], Box::new(of::AllOf)), &mut map);
    decouple_keyword((vec!["anyOf".to_string()], Box::new(of::AnyOf)), &mut map);
    decouple_keyword((vec!["oneOf".to_string()], Box::new(of::OneOf)), &mut map);
    decouple_keyword(
        (vec!["const".to_string()], Box::new(const_::Const)),
        &mut map,
    );
    decouple_keyword(
        (vec!["contains".to_string()], Box::new(contains::Contains)),
        &mut map,
    );
    decouple_keyword(
        (
            vec!["dependencies".to_string()],
            Box::new(dependencies::Dependencies),
        ),
        &mut map,
    );
    decouple_keyword((vec!["enum".to_string()], Box::new(enum_::Enum)), &mut map);
    decouple_keyword((vec!["not".to_string()], Box::new(not::Not)), &mut map);
    decouple_keyword(
        (vec!["items".to_string()], Box::new(items::Items)),
        &mut map,
    );
    decouple_keyword((vec!["$ref".to_string()], Box::new(ref_::Ref)), &mut map);
    decouple_keyword(
        (vec!["required".to_string()], Box::new(required::Required)),
        &mut map,
    );
    decouple_keyword((vec!["type".to_string()], Box::new(type_::Type)), &mut map);

    decouple_keyword(
        (
            vec!["exclusiveMaximum".to_string()],
            Box::new(maxmin::ExclusiveMaximum),
        ),
        &mut map,
    );
    decouple_keyword(
        (
            vec!["exclusiveMinimum".to_string()],
            Box::new(maxmin::ExclusiveMinimum),
        ),
        &mut map,
    );
    decouple_keyword(
        (
            vec!["maxItems".to_string()],
            Box::new(maxmin_items::MaxItems),
        ),
        &mut map,
    );
    decouple_keyword(
        (
            vec!["maxLength".to_string()],
            Box::new(maxmin_length::MaxLength),
        ),
        &mut map,
    );
    decouple_keyword(
        (
            vec!["maxProperties".to_string()],
            Box::new(maxmin_properties::MaxProperties),
        ),
        &mut map,
    );
    decouple_keyword(
        (vec!["maximum".to_string()], Box::new(maxmin::Maximum)),
        &mut map,
    );
    decouple_keyword(
        (
            vec!["minItems".to_string()],
            Box::new(maxmin_items::MinItems),
        ),
        &mut map,
    );
    decouple_keyword(
        (
            vec!["minLength".to_string()],
            Box::new(maxmin_length::MinLength),
        ),
        &mut map,
    );
    decouple_keyword(
        (
            vec!["minProperties".to_string()],
            Box::new(maxmin_properties::MinProperties),
        ),
        &mut map,
    );
    decouple_keyword(
        (vec!["minimum".to_string()], Box::new(maxmin::Minimum)),
        &mut map,
    );
    decouple_keyword(
        (vec!["pattern".to_string()], Box::new(pattern::Pattern)),
        &mut map,
    );
    decouple_keyword(
        (
            vec![
                "properties".to_string(),
                "additionalProperties".to_string(),
                "patternProperties".to_string(),
            ],
            Box::new(properties::Properties),
        ),
        &mut map,
    );
    decouple_keyword(
        (
            vec!["propertyNames".to_string()],
            Box::new(property_names::PropertyNames),
        ),
        &mut map,
    );
    decouple_keyword(
        (
            vec!["uniqueItems".to_string()],
            Box::new(unique_items::UniqueItems),
        ),
        &mut map,
    );

    map
}

pub fn decouple_keyword<V>(
    keyword_pair: KeywordPair<V>,
    map: &mut hashbrown::HashMap<String, Arc<KeywordConsumer<V>>>,
) where
    V: Value,
{
    let (keys, keyword) = keyword_pair;
    dbg!(keys.clone());

    let consumer = Arc::new(KeywordConsumer {
        keys: keys.clone(),
        keyword,
    });

    for key in keys.iter() {
        map.insert(key.to_string(), consumer.clone());
    }
}
