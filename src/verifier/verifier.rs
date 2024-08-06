use crate::models::credential::{CredentialResponse, VerifiablePresentation};
use crate::utils::crypto;
use crate::verifier::error::VerifierError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use log::{debug, error, info};
use serde_json::Value;

pub fn verify_credential(credential: &CredentialResponse) -> Result<bool, VerifierError> {
    info!("Verifying credential: {:?}", credential);

    if let Some(sd_jwt) = &credential.sd_jwt {
        verify_sd_jwt_credential(sd_jwt)
    } else {
        verify_vc_credential(credential)
    }
}

fn verify_vc_credential(credential: &CredentialResponse) -> Result<bool, VerifierError> {
    let credential_without_proof = {
        let mut cred = credential.clone();
        cred.proof = None;
        cred
    };
    info!("Credential without proof: {:?}", credential_without_proof);

    let proof = credential
        .proof
        .as_ref()
        .ok_or(VerifierError::MissingProof)?;
    info!("Proof: {:?}", proof);

    crypto::verify_vc(&credential_without_proof, proof).map_err(|e| {
        error!("Signature verification failed: {}", e);
        VerifierError::SignatureVerificationFailed(e.to_string())
    })?;

    if !is_trusted_issuer(&credential.issuer) {
        error!("Untrusted issuer: {}", credential.issuer);
        return Err(VerifierError::UntrustedIssuer);
    }

    Ok(true)
}

fn verify_sd_jwt_credential(sd_jwt: &str) -> Result<bool, VerifierError> {
    info!("Verifying SD-JWT: {}", sd_jwt);

    let parts: Vec<&str> = sd_jwt.split('.').collect();
    if parts.len() != 3 {
        return Err(VerifierError::InvalidCredentialFormat);
    }

    let payload_json = URL_SAFE_NO_PAD
        .decode(parts[1])
        .map_err(|_| VerifierError::InvalidBase64Encoding)?;
    let payload: Value =
        serde_json::from_slice(&payload_json).map_err(|_| VerifierError::InvalidJsonPayload)?;

    if payload.get("_sd_alg").is_none() {
        return Err(VerifierError::MissingSdAlgClaim);
    }

    crypto::verify_sd_jwt(sd_jwt).map_err(|e| {
        error!("SD-JWT verification failed: {}", e);
        VerifierError::SignatureVerificationFailed(e.to_string())
    })
}

pub fn verify_presentation(presentation: &VerifiablePresentation) -> Result<bool, VerifierError> {
    info!("Verifying presentation: {:?}", presentation);

    let presentation_without_proof = {
        let mut pres = presentation.clone();
        pres.proof = None;
        pres
    };
    info!(
        "Presentation without proof: {:?}",
        presentation_without_proof
    );

    let proof = presentation
        .proof
        .as_ref()
        .ok_or(VerifierError::MissingProof)?;
    info!("Presentation proof: {:?}", proof);

    crypto::verify_vc(&presentation_without_proof, proof).map_err(|e| {
        error!("Presentation signature verification failed: {}", e);
        VerifierError::SignatureVerificationFailed(e.to_string())
    })?;

    for credential in &presentation.verifiable_credential {
        info!("Verifying credential in presentation: {:?}", credential);
        verify_credential(credential)?;
    }

    Ok(true)
}

fn is_trusted_issuer(issuer: &str) -> bool {
    debug!("Checking if issuer is trusted: {}", issuer);
    // TODO: Implement actual issuer trust verification
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::issuer::issuer::create_sd_jwt_credential;
    use crate::models::credential::{CredentialResponse, VerifiablePresentation};
    use crate::models::sd_jwt::SDJWTCredentialRequest;
    use crate::utils::crypto;
    use crate::verifier::api;
    use actix_web::{test, web, App};
    use chrono::Utc;
    use log::{debug, info};

    fn create_test_credential() -> CredentialResponse {
        let mut credential = CredentialResponse {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            id: Some("http://example.edu/credentials/3732".to_string()),
            types: vec![
                "VerifiableCredential".to_string(),
                "UniversityDegreeCredential".to_string(),
            ],
            issuer: "did:example:123".to_string(),
            issuance_date: Utc::now().to_rfc3339(),
            credential_subject: serde_json::json!({
                "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
                "name": "Alice",
                "degree": {
                    "type": "BachelorDegree",
                    "name": "Bachelor of Science in Mechanical Engineering"
                }
            }),
            proof: None,
            sd_jwt: None,
            disclosures: None,
        };

        let credential_json = serde_json::to_value(&credential).unwrap();
        debug!("Credential to sign: {:?}", credential_json);
        let proof = crypto::sign_json(&credential_json).unwrap();
        credential.proof = Some(proof);
        debug!("Signed credential: {:?}", credential);
        credential
    }

    fn create_test_sd_jwt_credential() -> CredentialResponse {
        let request = SDJWTCredentialRequest {
            credential_subject: serde_json::json!({
                "given_name": "Alice",
                "family_name": "Smith",
                "email": "alice@example.com",
                "birthdate": "1990-01-01"
            }),
        };

        let sd_jwt_response = create_sd_jwt_credential(request).unwrap();

        // sd_jwt_response は既に CredentialResponse 型なので、そのまま返せます
        sd_jwt_response
    }

    #[actix_rt::test]
    async fn test_verify_credential() {
        // 通常のVC形式のテスト
        let credential = create_test_credential();
        info!("Standard Credential to verify: {:?}", credential);
        let result = verify_credential(&credential);
        info!("Standard Credential Verification result: {:?}", result);
        assert!(
            result.is_ok(),
            "Standard Credential Verification failed: {:?}",
            result.err()
        );
        assert!(result.unwrap());

        // SD-JWT形式のテスト
        let sd_jwt_credential = create_test_sd_jwt_credential();
        info!("SD-JWT Credential to verify: {:?}", sd_jwt_credential);
        let result = verify_credential(&sd_jwt_credential);
        info!("SD-JWT Credential Verification result: {:?}", result);
        assert!(
            result.is_ok(),
            "SD-JWT Credential Verification failed: {:?}",
            result.err()
        );
        assert!(result.unwrap());
    }

    #[actix_rt::test]
    async fn test_verify_presentation() {
        let credential = create_test_credential();
        let mut presentation = VerifiablePresentation {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            types: vec!["VerifiablePresentation".to_string()],
            verifiable_credential: vec![credential],
            proof: None,
        };

        let presentation_json = serde_json::to_value(&presentation).unwrap();
        info!("Presentation to sign: {:?}", presentation_json);
        let proof = crypto::sign_json(&presentation_json).unwrap();
        presentation.proof = Some(proof.clone());
        info!("Generated proof: {:?}", proof);

        info!("Presentation to verify: {:?}", presentation);
        let result = verify_presentation(&presentation);
        info!("Presentation verification result: {:?}", result);
        assert!(result.is_ok(), "Verification failed: {:?}", result.err());
        assert!(result.unwrap());
    }

    #[actix_rt::test]
    async fn test_verify_credential_api() {
        let app = test::init_service(App::new().service(
            web::resource("/verify/credential").route(web::post().to(api::verify_credential)),
        ))
        .await;

        // 通常のVC形式のテスト
        let credential = create_test_credential();
        info!("Standard Credential to verify via API: {:?}", credential);
        let req = test::TestRequest::post()
            .uri("/verify/credential")
            .set_json(&credential)
            .to_request();

        let resp = test::call_service(&app, req).await;
        info!("API response for standard credential: {:?}", resp);
        assert!(
            resp.status().is_success(),
            "API call failed for standard credential: {:?}",
            resp.status()
        );

        let body: serde_json::Value = test::read_body_json(resp).await;
        info!("API response body for standard credential: {:?}", body);
        assert_eq!(
            body["verified"], true,
            "Standard Credential Verification failed: {:?}",
            body
        );

        // SD-JWT形式のテスト
        let sd_jwt_credential = create_test_sd_jwt_credential();
        info!(
            "SD-JWT Credential to verify via API: {:?}",
            sd_jwt_credential
        );
        let req = test::TestRequest::post()
            .uri("/verify/credential")
            .set_json(&sd_jwt_credential)
            .to_request();

        let resp = test::call_service(&app, req).await;
        info!("API response for SD-JWT credential: {:?}", resp);
        assert!(
            resp.status().is_success(),
            "API call failed for SD-JWT credential: {:?}",
            resp.status()
        );

        let body: serde_json::Value = test::read_body_json(resp).await;
        info!("API response body for SD-JWT credential: {:?}", body);
        assert_eq!(
            body["verified"], true,
            "SD-JWT Credential Verification failed: {:?}",
            body
        );
    }

    #[actix_rt::test]
    async fn test_verify_presentation_api() {
        let app = test::init_service(App::new().service(
            web::resource("/verify/presentation").route(web::post().to(api::verify_presentation)),
        ))
        .await;

        let credential = create_test_credential();
        let mut presentation = VerifiablePresentation {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            types: vec!["VerifiablePresentation".to_string()],
            verifiable_credential: vec![credential],
            proof: None,
        };

        let presentation_json = serde_json::to_value(&presentation).unwrap();
        info!("Presentation to sign: {:?}", presentation_json);
        let proof = crypto::sign_json(&presentation_json).unwrap();
        presentation.proof = Some(proof);

        info!("Presentation to verify via API: {:?}", presentation);
        let req = test::TestRequest::post()
            .uri("/verify/presentation")
            .set_json(&presentation)
            .to_request();

        let resp = test::call_service(&app, req).await;
        info!("API response for presentation: {:?}", resp);
        assert!(
            resp.status().is_success(),
            "API call failed: {:?}",
            resp.status()
        );

        let body: serde_json::Value = test::read_body_json(resp).await;
        info!("API response body for presentation: {:?}", body);
        assert_eq!(body["verified"], true, "Verification failed: {:?}", body);
    }
}
