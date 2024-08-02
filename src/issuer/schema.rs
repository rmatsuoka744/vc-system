use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CredentialSchema {
    pub id: String,
    pub type_name: String,
    pub properties: HashMap<String, PropertyType>,
    pub required: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum PropertyType {
    String,
    Number,
    Boolean,
    Object,
    Array,
}

impl CredentialSchema {
    pub fn validate(&self, credential_subject: &Value) -> Result<(), String> {
        if let Value::Object(subject) = credential_subject {
            // Check required fields
            for field in &self.required {
                if !subject.contains_key(field) {
                    return Err(format!("Missing required field: {}", field));
                }
            }

            // Validate each property
            for (key, value) in subject {
                if let Some(property_type) = self.properties.get(key) {
                    match (property_type, value) {
                        (PropertyType::String, Value::String(_)) => {},
                        (PropertyType::Number, Value::Number(_)) => {},
                        (PropertyType::Boolean, Value::Bool(_)) => {},
                        (PropertyType::Object, Value::Object(_)) => {},
                        (PropertyType::Array, Value::Array(_)) => {},
                        _ => return Err(format!("Invalid type for field: {}", key)),
                    }
                } else {
                    return Err(format!("Unknown field: {}", key));
                }
            }
            Ok(())
        } else {
            Err("Credential subject must be an object".to_string())
        }
    }
}
