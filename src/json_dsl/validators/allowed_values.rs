use super::super::errors;
use simd_json::value::owned::Value as OwnedValue;

pub struct AllowedValues {
    allowed_values: Vec<OwnedValue>,
}

impl AllowedValues {
    pub fn new(values: Vec<OwnedValue>) -> AllowedValues {
        AllowedValues {
            allowed_values: values,
        }
    }
}

impl super::Validator for AllowedValues {
    fn validate(&self, val: &OwnedValue, path: &str) -> super::ValidatorResult {
        let mut matched = false;
        for allowed_value in self.allowed_values.iter() {
            if val == allowed_value {
                matched = true;
            }
        }

        if matched {
            Ok(())
        } else {
            Err(vec![Box::new(errors::WrongValue {
                path: path.to_string(),
                detail: Some("Value is not among allowed list".to_string()),
            })])
        }
    }
}
