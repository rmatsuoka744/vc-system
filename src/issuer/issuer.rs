use crate::models::credential::{CredentialRequest, CredentialResponse, IssuerMetadata};
use crate::utils::crypto;
use chrono::Utc;
use uuid::Uuid;

pub fn create_credential(request: CredentialRequest) -> Result<CredentialResponse, String> {
    // 1. Validate the request
    if !is_valid_credential_request(&request) {
        return Err("Invalid credential request".to_string());
    }

    // 2. Generate a unique identifier for the credential
    let credential_id = Uuid::new_v4().to_string();

    // 3. Create the credential
    let mut credential = CredentialResponse {
        context: request.context,
        id: Some(format!("http://example.edu/credentials/{}", credential_id)),
        credential_type: request.credential_type,
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
        Err(e) => Err(format!("Failed to sign credential: {}", e)),
    }
}

pub fn get_metadata() -> Result<IssuerMetadata, String> {
    // In a real-world scenario, this might be fetched from a database or configuration
    Ok(IssuerMetadata {
        id: "did:example:123".to_string(),
        name: "Example University".to_string(),
        public_key: crypto::get_public_key_info(),
    })
}

fn is_valid_credential_request(request: &CredentialRequest) -> bool {
    // Implement validation logic
    // For example, check if all required fields are present and in correct format
    !request.context.is_empty() && !request.credential_type.is_empty() && !request.issuer.is_empty()
}

fn sign_credential(credential: &CredentialResponse) -> Result<serde_json::Value, String> {
    // Implement signing logic using crypto utilities
    let credential_json = serde_json::to_value(credential)
        .map_err(|e| format!("Failed to serialize credential: {}", e))?;
    
    crypto::sign_json(&credential_json)
        .map_err(|e| format!("Failed to sign credential: {}", e))
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use crate::issuer::api;

    // 単体テスト
    #[test]
    async fn test_create_credential() {
        let request = CredentialRequest {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            credential_type: vec!["VerifiableCredential".to_string()],
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

    #[test]
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
                credential_type: vec!["VerifiableCredential".to_string()],
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