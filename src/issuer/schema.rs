use crate::issuer::error::IssuerError;
use crate::models::schema::{CredentialSchema, PropertyType};
use serde_json::Value;
use std::collections::HashMap;

pub fn get_schema(credential_type: &str) -> Option<CredentialSchema> {
    match credential_type {
        "UniversityDegreeCredential" => Some(CredentialSchema {
            id: "UniversityDegreeCredential".to_string(),
            type_name: "UniversityDegreeCredential".to_string(),
            properties: {
                let mut props = HashMap::new();
                props.insert("id".to_string(), PropertyType::String); // Add this line
                props.insert("name".to_string(), PropertyType::String);
                props.insert("degree".to_string(), PropertyType::Object);
                props
            },
            required: vec!["name".to_string(), "degree".to_string()],
        }),
        "SDJWTCredential" => Some(CredentialSchema {
            id: "SDJWTCredential".to_string(),
            type_name: "SDJWTCredential".to_string(),
            properties: {
                let mut props = HashMap::new();
                props.insert("id".to_string(), PropertyType::String); // Add this line
                props.insert("given_name".to_string(), PropertyType::String);
                props.insert("family_name".to_string(), PropertyType::String);
                props.insert("email".to_string(), PropertyType::String);
                props.insert("birthdate".to_string(), PropertyType::String);
                props
            },
            required: vec!["given_name".to_string(), "family_name".to_string()],
        }),
        _ => None,
    }
}

pub fn validate_credential_subject(
    subject: &Value,
    schema: &CredentialSchema,
) -> Result<(), IssuerError> {
    for field in &schema.required {
        if !subject.get(field).is_some() {
            return Err(IssuerError::SchemaValidationError(format!(
                "Missing required field: {}",
                field
            )));
        }
    }

    for (key, value) in subject.as_object().unwrap() {
        if let Some(property_type) = schema.properties.get(key) {
            match (property_type, value) {
                (PropertyType::String, Value::String(_)) => {}
                (PropertyType::Number, Value::Number(_)) => {}
                (PropertyType::Boolean, Value::Bool(_)) => {}
                (PropertyType::Object, Value::Object(_)) => {}
                (PropertyType::Array, Value::Array(_)) => {}
                _ => {
                    return Err(IssuerError::SchemaValidationError(format!(
                        "Invalid type for field: {}",
                        key
                    )))
                }
            }
        } else {
            return Err(IssuerError::SchemaValidationError(format!(
                "Unknown field: {}",
                key
            )));
        }
    }

    Ok(())
}
