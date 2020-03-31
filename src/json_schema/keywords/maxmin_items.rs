use simd_json::value::owned::Value as OwnedValue;
use value_trait::*;

use super::schema;
use super::validators;

kw_minmax_integer!(MaxItems, "maxItems");
kw_minmax_integer!(MinItems, "minItems");
