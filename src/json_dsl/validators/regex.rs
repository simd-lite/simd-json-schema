use regex;
use simd_json::value::owned::Value as OwnedValue;

use super::super::errors;

impl super::Validator for regex::Regex {
    fn validate(&self, val: &OwnedValue, path: &str) -> super::ValidatorResult {
        let string = strict_process!(val.as_str(), path, "The value must be a string");

        if self.is_match(string) {
            Ok(())
        } else {
            Err(vec![Box::new(errors::WrongValue {
                path: path.to_string(),
                detail: Some("Value is not matched by required pattern".to_string()),
            })])
        }
    }
}
