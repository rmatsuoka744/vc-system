use std::collections::HashMap;
use crate::models::schema::{CredentialSchema, PropertyType};

pub fn get_university_degree_schema() -> CredentialSchema {
    let mut properties = HashMap::new();
    properties.insert("id".to_string(), PropertyType::String);
    properties.insert("degree".to_string(), PropertyType::Object);

    CredentialSchema {
        id: "http://example.edu/schemas/degree.json".to_string(),
        type_name: "UniversityDegreeCredential".to_string(),
        properties,
        required: vec!["id".to_string(), "degree".to_string()],
    }
}

pub fn get_employment_credential_schema() -> CredentialSchema {
    let mut properties = HashMap::new();
    properties.insert("id".to_string(), PropertyType::String);
    properties.insert("employmentStatus".to_string(), PropertyType::String);
    properties.insert("employerName".to_string(), PropertyType::String);

    CredentialSchema {
        id: "http://example.com/schemas/employment.json".to_string(),
        type_name: "EmploymentCredential".to_string(),
        properties,
        required: vec!["id".to_string(), "employmentStatus".to_string(), "employerName".to_string()],
    }
}

pub fn get_schema(credential_type: &str) -> Option<CredentialSchema> {
    match credential_type {
        "UniversityDegreeCredential" => Some(get_university_degree_schema()),
        "EmploymentCredential" => Some(get_employment_credential_schema()),
        _ => None,
    }
}
