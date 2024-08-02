use crate::issuer::error::IssuerError;
use crate::issuer::schema;
use crate::models::credential::{CredentialRequest, CredentialResponse, IssuerMetadata};
use crate::utils::crypto;
use chrono::Utc;
use log::{debug, error, info};
use uuid::Uuid;

pub fn create_credential(request: CredentialRequest) -> Result<CredentialResponse, IssuerError> {
    info!("Received credential request: {:?}", request);

    let credential_type = request
        .types
        .iter()
        .find(|&t| t != "VerifiableCredential")
        .ok_or_else(|| {
            error!("No specific credential type provided");
            IssuerError::InvalidType("No specific credential type provided".to_string())
        })?;

    debug!("Credential type: {}", credential_type);

    let schema = schema::get_schema(credential_type).ok_or_else(|| {
        error!("Unsupported credential type: {}", credential_type);
        IssuerError::InvalidType(format!("Unsupported credential type: {}", credential_type))
    })?;

    debug!("Schema retrieved: {:?}", schema);

    schema::validate_credential_subject(&request.credential_subject, &schema)?;
    debug!("Credential subject validated successfully");

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
            info!("Credential signed successfully. ID: {}", credential_id);
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
    let public_key_info = crypto::get_public_key_info().map_err(|e| {
        error!("Failed to get public key info: {}", e);
        IssuerError::CryptoError(e)
    })?;

    let metadata = IssuerMetadata {
        id: "did:example:123".to_string(),
        name: "Example University".to_string(),
        public_key: public_key_info,
    };

    info!("Issuer metadata retrieved successfully");
    debug!("Issuer metadata: {:?}", metadata);

    Ok(metadata)
}

fn sign_credential(credential: &CredentialResponse) -> Result<serde_json::Value, IssuerError> {
    debug!("Signing credential");
    let credential_json = serde_json::to_value(credential).map_err(|e| {
        error!("Failed to serialize credential: {}", e);
        IssuerError::SerializationError(e.to_string())
    })?;

    debug!("Credential serialized to JSON successfully");

    let signed_credential = crypto::sign_json(&credential_json).map_err(|e| {
        error!("Failed to sign credential: {}", e);
        IssuerError::SigningError(e.to_string())
    })?;

    debug!("Credential signed successfully");
    Ok(signed_credential)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::issuer::api;
    use crate::models::credential::{CredentialRequest, CredentialResponse, IssuerMetadata};
    use actix_web::{test, web, App};
    use chrono::Utc;

    fn create_test_request() -> CredentialRequest {
        CredentialRequest {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            types: vec![
                "VerifiableCredential".to_string(),
                "UniversityDegreeCredential".to_string(),
            ],
            issuer: "did:example:123".to_string(),
            issuance_date: Utc::now().to_rfc3339(),
            credential_subject: serde_json::json!({
                "id": "did:example:456",
                "name": "Alice",
                "degree": {
                    "type": "BachelorDegree",
                    "name": "Bachelor of Science in Mechanical Engineering"
                }
            }),
        }
    }

    #[tokio::test]
    async fn test_create_credential() {
        let request = create_test_request();
        let result = create_credential(request);
        assert!(
            result.is_ok(),
            "Failed to create credential: {:?}",
            result.err()
        );

        let credential = result.unwrap();
        assert_eq!(credential.issuer, "did:example:123");
        assert!(credential.proof.is_some());
    }

    #[tokio::test]
    async fn test_get_metadata() {
        let result = get_metadata();
        assert!(result.is_ok(), "Failed to get metadata: {:?}", result.err());

        let metadata = result.unwrap();
        assert_eq!(metadata.id, "did:example:123");
        assert_eq!(metadata.name, "Example University");
        assert!(metadata.public_key.public_key_multibase.starts_with('z'));
    }

    #[actix_web::test]
    async fn test_issue_credential_api() {
        let app =
            test::init_service(App::new().service(
                web::resource("/credentials").route(web::post().to(api::issue_credential)),
            ))
            .await;

        let req = test::TestRequest::post()
            .uri("/credentials")
            .set_json(create_test_request())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(
            resp.status().is_success(),
            "API call failed: {:?}",
            resp.status()
        );

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
        assert!(
            resp.status().is_success(),
            "API call failed: {:?}",
            resp.status()
        );

        let body: IssuerMetadata = test::read_body_json(resp).await;
        assert_eq!(body.id, "did:example:123");
        assert_eq!(body.name, "Example University");
        assert!(body.public_key.public_key_multibase.starts_with('z'));
    }
}
