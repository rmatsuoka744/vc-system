use crate::models::credential::{CredentialResponse, VerifiablePresentation};
use crate::utils::crypto;

pub fn verify_credential(credential: &CredentialResponse) -> Result<bool, String> {
    // 1. Check if the issuer is trusted (this would typically involve checking against a list of trusted issuers)
    if !is_trusted_issuer(&credential.issuer) {
        return Ok(false);
    }

    // 2. Verify the credential's signature
    match &credential.proof {
        Some(proof) => crypto::verify_signature(&credential, proof),
        None => Ok(false),
    }
}

pub fn verify_presentation(presentation: &VerifiablePresentation) -> Result<bool, String> {
    // 1. Verify the presentation's signature
    let presentation_valid = match &presentation.proof {
        Some(proof) => crypto::verify_signature(presentation, proof)?,
        None => false,
    };

    if !presentation_valid {
        return Ok(false);
    }

    // 2. Verify each credential in the presentation
    for credential in &presentation.verifiable_credential {
        if !verify_credential(credential)? {
            return Ok(false);
        }
    }

    Ok(true)
}

fn is_trusted_issuer(_issuer: &str) -> bool {
    // 実装...
    true
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::credential::{CredentialResponse, VerifiablePresentation};
    use actix_web::{test, web, App};
    use crate::verifier::api;

    fn create_test_credential() -> CredentialResponse {
        CredentialResponse {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            id: Some("http://example.edu/credentials/3732".to_string()),
            credential_type: vec!["VerifiableCredential".to_string()],
            issuer: "https://example.edu/issuers/14".to_string(),
            issuance_date: "2010-01-01T19:23:24Z".to_string(),
            credential_subject: serde_json::json!({"id": "did:example:ebfeb1f712ebc6f1c276e12ec21", "degree": {"type": "BachelorDegree", "name": "Bachelor of Science in Mechanical Engineering"}}),
            proof: Some(serde_json::json!({
                "type": "Ed25519Signature2018",
                "created": "2021-03-19T15:30:15Z",
                "jws": "eyJhbGciOiJFZERTQSIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19..PT8yCqVjj5ZHD1MkV4OtQXc-oa0_Azf9OaDPyHKQWALb3b-j9wj5oO2-RZvB0Lr_uy7yFXwAl6bwqEDP8YkTAQ"
            })),
        }
    }

    #[test]
    async fn test_verify_credential() {
        let credential = create_test_credential();
        let result = verify_credential(&credential);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    async fn test_verify_presentation() {
        let credential = create_test_credential();
        let presentation = VerifiablePresentation {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            presentation_type: vec!["VerifiablePresentation".to_string()],
            verifiable_credential: vec![credential],
            proof: Some(serde_json::json!({
                "type": "Ed25519Signature2018",
                "created": "2021-03-19T15:30:15Z",
                "challenge": "1f44d55f-f161-4938-a659-f8026467f126",
                "domain": "example.com",
                "jws": "eyJhbGciOiJFZERTQSIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19..PT8yCqVjj5ZHD1MkV4OtQXc-oa0_Azf9OaDPyHKQWALb3b-j9wj5oO2-RZvB0Lr_uy7yFXwAl6bwqEDP8YkTAQ"
            })),
        };
        let result = verify_presentation(&presentation);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[actix_web::test]
    async fn test_verify_credential_api() {
        let app = test::init_service(
            App::new().service(web::resource("/verify/credential").route(web::post().to(api::verify_credential)))
        ).await;

        let credential = create_test_credential();
        let req = test::TestRequest::post()
            .uri("/verify/credential")
            .set_json(&credential)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["verified"], true);
    }

    #[actix_web::test]
    async fn test_verify_presentation_api() {
        let app = test::init_service(
            App::new().service(web::resource("/verify/presentation").route(web::post().to(api::verify_presentation)))
        ).await;

        let credential = create_test_credential();
        let presentation = VerifiablePresentation {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            presentation_type: vec!["VerifiablePresentation".to_string()],
            verifiable_credential: vec![credential],
            proof: Some(serde_json::json!({
                "type": "Ed25519Signature2018",
                "created": "2021-03-19T15:30:15Z",
                "challenge": "1f44d55f-f161-4938-a659-f8026467f126",
                "domain": "example.com",
                "jws": "eyJhbGciOiJFZERTQSIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19..PT8yCqVjj5ZHD1MkV4OtQXc-oa0_Azf9OaDPyHKQWALb3b-j9wj5oO2-RZvB0Lr_uy7yFXwAl6bwqEDP8YkTAQ"
            })),
        };

        let req = test::TestRequest::post()
            .uri("/verify/presentation")
            .set_json(&presentation)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["verified"], true);
    }
}
