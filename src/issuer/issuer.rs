use crate::models::credential::{CredentialRequest, CredentialResponse, IssuerMetadata};
use crate::utils::crypto;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::issuer::error::IssuerError;
use url::Url;

pub fn create_credential(request: CredentialRequest) -> Result<CredentialResponse, IssuerError> {
    // 1. Validate the request
    validate_credential_request(&request)?;

    // 2. Generate a unique identifier for the credential
    let credential_id = Uuid::new_v4().to_string();

    // 3. Create the credential
    let mut credential = CredentialResponse {
        context: request.context,
        id: Some(format!("http://example.edu/credentials/{}", credential_id)),
        types: request.types,
        issuer: request.issuer,
        issuance_date: Utc::now().to_rfc3339(),
        credential_subject: request.credential_subject,
        proof: None,
    };

    // 4. Sign the credential
    match sign_credential(&credential) {
        Ok(proof) => {
            credential.proof = Some(proof);
            Ok(credential)
        }
        Err(e) => Err(e),
    }
}

pub fn get_metadata() -> Result<IssuerMetadata, IssuerError> {
    // In a real-world scenario, this might be fetched from a database or configuration
    Ok(IssuerMetadata {
        id: "did:example:123".to_string(),
        name: "Example University".to_string(),
        public_key: crypto::get_public_key_info(),
    })
}

pub fn validate_credential_request(request: &CredentialRequest) -> Result<(), IssuerError> {
    // コンテキストの検証
    if !request.context.contains(&"https://www.w3.org/2018/credentials/v1".to_string()) {
        return Err(IssuerError::InvalidContext(
            "must include 'https://www.w3.org/2018/credentials/v1'".to_string(),
        ));
    }
    // タイプの検証
    if !request.types.contains(&"VerifiableCredential".to_string()) {
        return Err(IssuerError::InvalidType(
            "must include 'VerifiableCredential'".to_string(),
        ));
    }
    // 発行者の検証
    if Url::parse(&request.issuer).is_err() {
        return Err(IssuerError::InvalidIssuer(
            "must be a valid URL".to_string(),
        ));
    }
    // 発行日の検証
    match DateTime::parse_from_rfc3339(&request.issuance_date) {
        Ok(date) => {
            if date > Utc::now() {
                return Err(IssuerError::InvalidIssuanceDate(
                    "cannot be in the future".to_string(),
                ));
            }
        }
        Err(_) => {
            return Err(IssuerError::InvalidIssuanceDate(
                "must be a valid RFC 3339 date".to_string(),
            ));
        }
    }
    // credential_subject の検証
    if !request.credential_subject.is_object() {
        return Err(IssuerError::InvalidCredentialSubject(
            "must be a JSON object".to_string(),
        ));
    }
    Ok(())
}

fn sign_credential(credential: &CredentialResponse) -> Result<serde_json::Value, IssuerError> {
    let credential_json = serde_json::to_value(credential)
        .map_err(|e| IssuerError::SerializationError(e.to_string()))?;

        crypto::sign_json(&credential_json)
        .map_err(|e| IssuerError::SigningError(e.to_string()))
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
            types: vec!["VerifiableCredential".to_string()],
            issuer: "did:example:123".to_string(),
            issuance_date: Utc::now().to_rfc3339(),
            credential_subject: serde_json::json!({"id": "did:example:456", "name": "Alice"}),
        };

        let result = create_credential(request);
        assert!(result.is_ok());

        let credential = result.unwrap();
        assert_eq!(credential.issuer, "did:example:123");
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
        let app =
            test::init_service(App::new().service(
                web::resource("/credentials").route(web::post().to(api::issue_credential)),
            ))
            .await;

        let req = test::TestRequest::post()
            .uri("/credentials")
            .set_json(CredentialRequest {
                context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
                types: vec!["VerifiableCredential".to_string()],
                issuer: "did:example:123".to_string(),
                issuance_date: Utc::now().to_rfc3339(),
                credential_subject: serde_json::json!({"id": "did:example:456", "name": "Alice"}),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: CredentialResponse = test::read_body_json(resp).await;
        assert_eq!(body.issuer, "did:example:123");
        assert!(body.proof.is_some());
    }

    #[actix_web::test]
    async fn test_get_issuer_metadata_api() {
        let app = test::init_service(
            App::new()
                .service(web::resource("/metadata").route(web::get().to(api::get_issuer_metadata))),
        )
        .await;

        let req = test::TestRequest::get().uri("/metadata").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: IssuerMetadata = test::read_body_json(resp).await;
        assert_eq!(body.id, "did:example:123");
        assert_eq!(body.name, "Example University");
    }
}
