use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

use super::super::schema;
use super::super::validators;

kw_minmax_integer!(MaxProperties, "maxProperties");
kw_minmax_integer!(MinProperties, "minProperties");
