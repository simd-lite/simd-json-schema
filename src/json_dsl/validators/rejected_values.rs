use simd_json::value::owned::Value as OwnedValue;

use super::super::errors;

pub struct RejectedValues {
    rejected_values: Vec<OwnedValue>,
}

impl RejectedValues {
    pub fn new(values: Vec<OwnedValue>) -> RejectedValues {
        RejectedValues {
            rejected_values: values,
        }
    }
}

impl super::Validator for RejectedValues {
    fn validate(&self, val: &OwnedValue, path: &str) -> super::ValidatorResult {
        let mut matched = false;
        for rejected_value in self.rejected_values.iter() {
            if val == rejected_value {
                matched = true;
            }
        }

        if matched {
            Err(vec![Box::new(errors::WrongValue {
                path: path.to_string(),
                detail: Some("Value is among reject list".to_string()),
            })])
        } else {
            Ok(())
        }
    }
}
