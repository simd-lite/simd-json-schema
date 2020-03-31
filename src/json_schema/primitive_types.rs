use std::fmt;
use std::str;

#[derive(Copy, Debug, Clone)]
pub enum PrimitiveType {
    Array,
    Boolean,
    Integer,
    Number,
    Null,
    Object,
    String,
}

impl str::FromStr for PrimitiveType {
    type Err = ();
    fn from_str(s: &str) -> Result<PrimitiveType, ()> {
        match s {
            "array" => Ok(PrimitiveType::Array),
            "boolean" => Ok(PrimitiveType::Boolean),
            "integer" => Ok(PrimitiveType::Integer),
            "number" => Ok(PrimitiveType::Number),
            "null" => Ok(PrimitiveType::Null),
            "object" => Ok(PrimitiveType::Object),
            "string" => Ok(PrimitiveType::String),
            _ => Err(()),
        }
    }
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(match self {
            PrimitiveType::Array => "array",
            PrimitiveType::Boolean => "boolean",
            PrimitiveType::Integer => "integer",
            PrimitiveType::Number => "number",
            PrimitiveType::Null => "null",
            PrimitiveType::Object => "object",
            PrimitiveType::String => "string",
        })
    }
}
