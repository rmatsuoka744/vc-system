use crate::models::schema::{CredentialSchema, PropertyType};
use crate::issuer::error::IssuerError;
use serde_json::Value;

pub fn get_schema(credential_type: &str) -> Option<CredentialSchema> {
    match credential_type {
        "UniversityDegreeCredential" => Some(crate::models::schema::get_university_degree_schema()),
        "EmploymentCredential" => Some(crate::models::schema::get_employment_credential_schema()),
        _ => None,
    }
}

pub fn validate_credential_subject(subject: &Value, schema: &CredentialSchema) -> Result<(), IssuerError> {
    for field in &schema.required {
        if !subject.get(field).is_some() {
            return Err(IssuerError::SchemaValidationError(format!("Missing required field: {}", field)));
        }
    }

    for (key, value) in subject.as_object().unwrap() {
        if let Some(property_type) = schema.properties.get(key) {
            match (property_type, value) {
                (PropertyType::String, Value::String(_)) => {},
                (PropertyType::Number, Value::Number(_)) => {},
                (PropertyType::Boolean, Value::Bool(_)) => {},
                (PropertyType::Object, Value::Object(_)) => {},
                (PropertyType::Array, Value::Array(_)) => {},
                _ => return Err(IssuerError::SchemaValidationError(format!("Invalid type for field: {}", key))),
            }
        } else {
            return Err(IssuerError::SchemaValidationError(format!("Unknown field: {}", key)));
        }
    }

    Ok(())
}
