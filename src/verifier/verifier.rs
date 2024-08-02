use crate::models::credential::{CredentialResponse, VerifiablePresentation};
use crate::utils::crypto;
use log::{debug, info, warn, error};

pub fn verify_credential(credential: &CredentialResponse) -> Result<bool, String> {
    info!(
        "Starting verification of credential: {}",
        credential.id.as_ref().unwrap_or(&"Unknown ID".to_string())
    );
    debug!("Verifying credential: {:?}", credential);

    let credential_without_proof = {
        let mut cred = credential.clone();
        cred.proof = None;
        cred
    };

    debug!("Credential without proof: {:?}", credential_without_proof);

    let proof = credential.proof.as_ref().ok_or_else(|| {
        error!("Proof is missing in the credential");
        "Proof is missing".to_string()
    })?;

    debug!("Verifying signature");
    match crypto::verify_signature(&credential_without_proof, proof) {
        Ok(_) => debug!("Signature verification successful"),
        Err(e) => {
            error!("Signature verification failed: {}", e);
            return Err(e);
        }
    }

    // 2. 発行者の検証 (ここでは簡略化していますが、実際にはDIDの解決などが必要です)
    if !is_trusted_issuer(&credential.issuer) {
        warn!("Untrusted issuer: {}", credential.issuer);
        return Err("Untrusted issuer".to_string());
    }
    debug!("Issuer verification successful");

    // 3. その他の検証 (有効期限、失効状態など)
    // TODO: Implement additional verifications
    debug!("Additional verifications not implemented yet");

    info!("Credential verification successful");
    Ok(true)
}

pub fn verify_presentation(presentation: &VerifiablePresentation) -> Result<bool, String> {
    info!("Starting verification of presentation");
    debug!("Verifying presentation: {:?}", presentation);

    let presentation_without_proof = {
        let mut pres = presentation.clone();
        pres.proof = None;
        pres
    };

    debug!(
        "Presentation without proof: {:?}",
        presentation_without_proof
    );

    let proof = presentation.proof.as_ref().ok_or_else(|| {
        error!("Proof is missing in the presentation");
        "Proof is missing".to_string()
    })?;

    debug!("Verifying presentation signature");
    match crypto::verify_signature(&presentation_without_proof, proof) {
        Ok(_) => debug!("Presentation signature verification successful"),
        Err(e) => {
            error!("Presentation signature verification failed: {}", e);
            return Err(e);
        }
    }

    // 2. 含まれる各資格情報の検証
    debug!("Verifying individual credentials in the presentation");
    for (index, credential) in presentation.verifiable_credential.iter().enumerate() {
        info!("Verifying credential {} in the presentation", index + 1);
        match verify_credential(credential) {
            Ok(_) => debug!("Credential {} verification successful", index + 1),
            Err(e) => {
                error!("Credential {} verification failed: {}", index + 1, e);
                return Err(format!(
                    "Credential {} verification failed: {}",
                    index + 1,
                    e
                ));
            }
        }
    }

    info!("Presentation verification successful");
    Ok(true)
}

fn is_trusted_issuer(issuer: &str) -> bool {
    // 実際の実装では、信頼できる発行者のリストをチェックします
    debug!("Checking if issuer is trusted: {}", issuer);
    // TODO: Implement actual issuer trust verification
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::credential::{CredentialResponse, VerifiablePresentation};
    use crate::utils::crypto;
    use crate::verifier::api;
    use actix_web::{test, web, App};
    use chrono::Utc;
    use log::debug;

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
        };

        let credential_json = serde_json::to_value(&credential).unwrap();
        debug!("Credential to sign: {:?}", credential_json);
        let proof = crypto::sign_json(&credential_json).unwrap();
        credential.proof = Some(proof);
        debug!("Signed credential: {:?}", credential);
        credential
    }

    #[actix_rt::test]
    async fn test_verify_credential() {
        let credential = create_test_credential();
        debug!("Credential to verify: {:?}", credential);
        let result = verify_credential(&credential);
        debug!("Verification result: {:?}", result);
        assert!(result.is_ok(), "Verification failed: {:?}", result.err());
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
        debug!("Presentation to sign: {:?}", presentation_json);
        let proof = crypto::sign_json(&presentation_json).unwrap();
        presentation.proof = Some(proof);

        debug!("Presentation to verify: {:?}", presentation);
        let result = verify_presentation(&presentation);
        assert!(result.is_ok(), "Verification failed: {:?}", result.err());
        assert!(result.unwrap());
    }

    #[actix_rt::test]
    async fn test_verify_credential_api() {
        let app = test::init_service(App::new().service(
            web::resource("/verify/credential").route(web::post().to(api::verify_credential)),
        ))
        .await;

        let credential = create_test_credential();
        debug!("Credential to verify via API: {:?}", credential);
        let req = test::TestRequest::post()
            .uri("/verify/credential")
            .set_json(&credential)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(
            resp.status().is_success(),
            "API call failed: {:?}",
            resp.status()
        );

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["verified"], true, "Verification failed: {:?}", body);
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
        debug!("Presentation to sign: {:?}", presentation_json);
        let proof = crypto::sign_json(&presentation_json).unwrap();
        presentation.proof = Some(proof);

        debug!("Presentation to verify via API: {:?}", presentation);
        let req = test::TestRequest::post()
            .uri("/verify/presentation")
            .set_json(&presentation)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(
            resp.status().is_success(),
            "API call failed: {:?}",
            resp.status()
        );

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["verified"], true, "Verification failed: {:?}", body);
    }
}
