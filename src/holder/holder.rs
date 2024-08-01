use std::sync::Arc;
use crate::models::credential::{CredentialResponse, PresentationRequest, VerifiablePresentation};
use super::storage::Storage;

#[derive(Clone)]
pub struct Holder {
    storage: Arc<dyn Storage>,
}

impl Holder {
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Holder { storage }
    }

    pub fn store_credential(&self, credential: CredentialResponse) -> Result<String, String> {
        let id = uuid::Uuid::new_v4().to_string();
        self.storage.store(id.clone(), credential)?;
        Ok(id)
    }

    pub fn get_credentials(&self) -> Result<Vec<CredentialResponse>, String> {
        self.storage.get_all()
    }

    pub fn create_presentation(&self, request: PresentationRequest) -> Result<VerifiablePresentation, String> {
        let mut selected_credentials = Vec::new();
        for id in &request.credential_ids {
            if let Some(credential) = self.storage.get(id)? {
                selected_credentials.push(credential);
            } else {
                return Err(format!("Credential with id {} not found", id));
            }
        }

        Ok(VerifiablePresentation {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            presentation_type: vec!["VerifiablePresentation".to_string()],
            verifiable_credential: selected_credentials,
            proof: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holder::storage::test_storage::TestStorage;
    use actix_web::{test, web, App};
    use crate::holder::api;

    fn setup_test_holder() -> Holder {
        Holder::new(Arc::new(TestStorage::new()))
    }

    #[actix_web::test]
    async fn test_store_credential() {
        let holder = setup_test_holder();

        let credential = CredentialResponse {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            id: Some("test_id".to_string()),
            credential_type: vec!["VerifiableCredential".to_string()],
            issuer: "did:example:123".to_string(),
            issuance_date: "2023-01-01T00:00:00Z".to_string(),
            credential_subject: serde_json::json!({"id": "did:example:456", "name": "Alice"}),
            proof: None,
        };

        let result = holder.store_credential(credential.clone());
        assert!(result.is_ok());

        let stored_credentials = holder.get_credentials().unwrap();
        assert_eq!(stored_credentials.len(), 1);
        assert_eq!(stored_credentials[0].issuer, "did:example:123");
    }

    #[actix_web::test]
    async fn test_create_presentation() {
        let holder = setup_test_holder();

        let credential = CredentialResponse {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            id: Some("test_id".to_string()),
            credential_type: vec!["VerifiableCredential".to_string()],
            issuer: "did:example:123".to_string(),
            issuance_date: "2023-01-01T00:00:00Z".to_string(),
            credential_subject: serde_json::json!({"id": "did:example:456", "name": "Alice"}),
            proof: None,
        };

        let credential_id = holder.store_credential(credential).unwrap();

        let request = PresentationRequest {
            credential_ids: vec![credential_id],
            challenge: "challenge".to_string(),
            domain: "example.com".to_string(),
        };

        let result = holder.create_presentation(request);
        assert!(result.is_ok());

        let presentation = result.unwrap();
        assert_eq!(presentation.verifiable_credential.len(), 1);
        assert_eq!(presentation.verifiable_credential[0].issuer, "did:example:123");
    }

    #[actix_web::test]
    async fn test_store_credential_api() {
        let holder = Arc::new(setup_test_holder());

        let app = test::init_service(
            App::new().app_data(web::Data::new(holder.clone()))
                .service(web::resource("/credentials").route(web::post().to(api::store_credential)))
        ).await;

        let credential = CredentialResponse {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            id: Some("test_id".to_string()),
            credential_type: vec!["VerifiableCredential".to_string()],
            issuer: "did:example:123".to_string(),
            issuance_date: "2023-01-01T00:00:00Z".to_string(),
            credential_subject: serde_json::json!({"id": "did:example:456", "name": "Alice"}),
            proof: None,
        };

        let req = test::TestRequest::post()
            .uri("/credentials")
            .set_json(&credential)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body.get("id").is_some());
        assert_eq!(body["status"], "stored");
    }

    #[actix_web::test]
    async fn test_get_credentials_api() {
        let holder = Arc::new(setup_test_holder());

        let app = test::init_service(
            App::new().app_data(web::Data::new(holder.clone()))
                .service(web::resource("/credentials").route(web::get().to(api::get_credentials)))
        ).await;

        // First, store a credential
        let credential = CredentialResponse {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            id: Some("test_id".to_string()),
            credential_type: vec!["VerifiableCredential".to_string()],
            issuer: "did:example:123".to_string(),
            issuance_date: "2023-01-01T00:00:00Z".to_string(),
            credential_subject: serde_json::json!({"id": "did:example:456", "name": "Alice"}),
            proof: None,
        };
        holder.store_credential(credential).unwrap();

        let req = test::TestRequest::get().uri("/credentials").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: Vec<CredentialResponse> = test::read_body_json(resp).await;
        assert_eq!(body.len(), 1);
        assert_eq!(body[0].issuer, "did:example:123");
    }
}
