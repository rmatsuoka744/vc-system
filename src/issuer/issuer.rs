use log::{info, warn, error, debug};
use crate::models::credential::{CredentialRequest, CredentialResponse, IssuerMetadata};
use crate::utils::crypto;
use crate::issuer::error::IssuerError;
use crate::issuer::schema::{CredentialSchema, PropertyType};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use url::Url;
use std::collections::HashMap;

pub fn create_credential(request: CredentialRequest) -> Result<CredentialResponse, IssuerError> {
    info!("Received credential request: {:?}", request);

    // リクエストからcredential typeを取得
    let credential_type = request.types.iter()
        .find(|&t| t != "VerifiableCredential")
        .ok_or_else(|| {
            error!("No specific credential type provided");
            IssuerError::InvalidType("No specific credential type provided".to_string())
        })?;

    debug!("Credential type: {}", credential_type);

    // スキーマを取得
    let schema = get_schema(credential_type)
        .ok_or_else(|| {
            error!("Unsupported credential type: {}", credential_type);
            IssuerError::InvalidType(format!("Unsupported credential type: {}", credential_type))
        })?;

    debug!("Schema retrieved: {:?}", schema);

    // リクエストの検証
    validate_credential_request(&request, &schema)?;

    info!("Credential request validated successfully");

    let credential_id = Uuid::new_v4().to_string();
    debug!("Generated credential ID: {}", credential_id);

    let mut credential = CredentialResponse {
        context: request.context,
        id: Some(format!("http://example.edu/credentials/{}", credential_id)),
        types: request.types,
        issuer: request.issuer,
        issuance_date: Utc::now().to_rfc3339(),
        credential_subject: request.credential_subject,
        proof: None,
    };

    debug!("Created credential: {:?}", credential);

    match sign_credential(&credential) {
        Ok(proof) => {
            credential.proof = Some(proof);
            info!("Credential signed successfully");
            Ok(credential)
        }
        Err(e) => {
            error!("Failed to sign credential: {:?}", e);
            Err(e)
        }
    }
}

pub fn get_metadata() -> Result<IssuerMetadata, IssuerError> {
    debug!("Fetching issuer metadata");
    // In a real-world scenario, this might be fetched from a database or configuration
    Ok(IssuerMetadata {
        id: "did:example:123".to_string(),
        name: "Example University".to_string(),
        public_key: crypto::get_public_key_info(),
    })
}

// スキーマを取得する関数
fn get_schema(credential_type: &str) -> Option<CredentialSchema> {
    debug!("Getting schema for credential type: {}", credential_type);
    match credential_type {
        "UniversityDegreeCredential" => Some(create_university_degree_schema()),
        // 他のcredential typeに対するスキーマをここに追加
        _ => {
            warn!("No schema found for credential type: {}", credential_type);
            None
        }
    }
}

fn create_university_degree_schema() -> CredentialSchema {
    debug!("Creating University Degree schema");
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


fn validate_credential_request(request: &CredentialRequest, schema: &CredentialSchema) -> Result<(), IssuerError> {
    debug!("Validating credential request");

    if !request.context.contains(&"https://www.w3.org/2018/credentials/v1".to_string()) {
        error!("Invalid context: must include 'https://www.w3.org/2018/credentials/v1'");
        return Err(IssuerError::InvalidContext(
            "must include 'https://www.w3.org/2018/credentials/v1'".to_string(),
        ));
    }
    if !request.types.contains(&"VerifiableCredential".to_string()) {
        error!("Invalid type: must include 'VerifiableCredential'");
        return Err(IssuerError::InvalidType(
            "must include 'VerifiableCredential'".to_string(),
        ));
    }
    if Url::parse(&request.issuer).is_err() {
        error!("Invalid issuer: must be a valid URL");
        return Err(IssuerError::InvalidIssuer(
            "must be a valid URL".to_string(),
        ));
    }
    match DateTime::parse_from_rfc3339(&request.issuance_date) {
        Ok(date) => {
            if date > Utc::now() {
                error!("Invalid issuance date: cannot be in the future");
                return Err(IssuerError::InvalidIssuanceDate(
                    "cannot be in the future".to_string(),
                ));
            }
        }
        Err(_) => {
            error!("Invalid issuance date: must be a valid RFC 3339 date");
            return Err(IssuerError::InvalidIssuanceDate(
                "must be a valid RFC 3339 date".to_string(),
            ));
        }
    }

    // スキーマ検証
    if let Err(e) = schema.validate(&request.credential_subject) {
        error!("Schema validation error: {}. Expected fields: id, degree", e);
        return Err(IssuerError::SchemaValidationError(e));
    }

    info!("Credential request validated successfully");
    Ok(())
}

fn sign_credential(credential: &CredentialResponse) -> Result<serde_json::Value, IssuerError> {
    debug!("Signing credential");
    let credential_json = serde_json::to_value(credential)
        .map_err(|e| {
            error!("Failed to serialize credential: {}", e);
            IssuerError::SerializationError(e.to_string())
        })?;

    crypto::sign_json(&credential_json)
        .map_err(|e| {
            error!("Failed to sign credential: {}", e);
            IssuerError::SigningError(e.to_string())
        })
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::issuer::api;
    use actix_web::{test, web, App};

    // 単体テスト
    #[tokio::test]
    async fn test_create_credential() {
        let request = CredentialRequest {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            types: vec!["VerifiableCredential".to_string(), "UniversityDegreeCredential".to_string()],
            issuer: "https://example.edu/issuers/14".to_string(),
            issuance_date: Utc::now().to_rfc3339(),
            credential_subject: serde_json::json!({
                "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
                "name": "Jane Doe",
                "degree": {
                    "type": "BachelorDegree",
                    "name": "Bachelor of Science in Mechanical Engineering"
                }
            }),
        };

        let result = create_credential(request);
        assert!(result.is_ok());

        let credential = result.unwrap();
        assert_eq!(credential.issuer, "https://example.edu/issuers/14");
        assert!(credential.proof.is_some());
    }

    #[tokio::test]
    async fn test_get_metadata() {
        let result = get_metadata();
        assert!(result.is_ok());

        let metadata = result.unwrap();
        assert_eq!(metadata.id, "did:example:123");
        assert_eq!(metadata.name, "Example University");
    }

    // 統合テスト（APIを使用）
    #[actix_web::test]
    async fn test_issue_credential_api() {
        let app = test::init_service(
            App::new().service(web::resource("/credentials").route(web::post().to(api::issue_credential)))
        ).await;

        let req = test::TestRequest::post()
            .uri("/credentials")
            .set_json(CredentialRequest {
                context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
                types: vec!["VerifiableCredential".to_string(), "UniversityDegreeCredential".to_string()],
                issuer: "https://example.edu/issuers/14".to_string(),
                issuance_date: Utc::now().to_rfc3339(),
                credential_subject: serde_json::json!({
                    "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
                    "name": "Jane Doe",
                    "degree": {
                        "type": "BachelorDegree",
                        "name": "Bachelor of Science in Mechanical Engineering"
                    }
                }),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: CredentialResponse = test::read_body_json(resp).await;
        assert_eq!(body.issuer, "https://example.edu/issuers/14");
        assert!(body.proof.is_some());
    }

    #[actix_web::test]
    async fn test_get_issuer_metadata_api() {
        let app = test::init_service(
            App::new().service(web::resource("/metadata").route(web::get().to(api::get_issuer_metadata)))
        ).await;

        let req = test::TestRequest::get().uri("/metadata").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: IssuerMetadata = test::read_body_json(resp).await;
        assert_eq!(body.id, "did:example:123");
        assert_eq!(body.name, "Example University");
    }
}
