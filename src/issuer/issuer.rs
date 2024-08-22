use crate::issuer::error::IssuerError;
use crate::issuer::schema;
use crate::models::credential::{CredentialRequest, CredentialResponse, IssuerMetadata};
use crate::models::schema::CredentialSchema;
use crate::models::sd_jwt::{SDJWTCredentialRequest, SDJWTCredentialResponse};
use crate::utils::{crypto, sd_jwt};
use chrono::Utc;
use log::{debug, error, info};
use serde_json::json;
use uuid::Uuid;

pub fn create_credential(request: CredentialRequest) -> Result<CredentialResponse, IssuerError> {
    info!("Received credential request: {:?}", request);

    let credential_type = get_credential_type(&request.types)?;
    let schema = get_schema(credential_type)?;

    schema::validate_credential_subject(&request.credential_subject, &schema)
        .map_err(|e| IssuerError::SchemaValidationError(e.to_string()))?;
    debug!("Credential subject validated successfully");

    let credential = create_unsigned_credential(request)?;
    sign_and_finalize_credential(credential)
}

pub fn create_sd_jwt_credential(
    request: SDJWTCredentialRequest,
) -> Result<CredentialResponse, IssuerError> {
    info!("Creating SD-JWT credential");

    let (sd_jwt, disclosures) = create_sd_jwt(&request)?;

    // VCリクエストの構造をクライアントに設定させる項目のみ含める
    let vc_request = CredentialRequest {
        context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
        types: vec![
            "VerifiableCredential".to_string(),
            "SDJWTCredential".to_string(),
        ],
        issuer: "".to_string(),        // Issuer側で設定するため空にする
        issuance_date: "".to_string(), // Issuer側で設定するため空にする
        credential_subject: request.credential_subject.clone(),
    };

    let mut vc = create_credential(vc_request)?;

    // SD-JWT と開示情報を追加
    vc.sd_jwt = Some(sd_jwt);
    vc.disclosures = Some(disclosures);

    Ok(vc)
}

pub fn create_sd_jwt_vc(
    request: SDJWTCredentialRequest,
) -> Result<SDJWTCredentialResponse, IssuerError> {
    let mut vc = create_sd_jwt_credential(request)?;

    // `CredentialResponse` から `sd_jwt` と `disclosures` を取り出し、`CredentialResponse` から削除
    let sd_jwt = vc.sd_jwt.take().unwrap_or_default();
    let disclosures = vc.disclosures.take().unwrap_or_default();

    // `CredentialResponse` から `sd_jwt` と `disclosures` を削除
    vc.sd_jwt = None;
    vc.disclosures = None;

    // `SDJWTCredentialResponse` を作成して返す
    Ok(SDJWTCredentialResponse {
        verifiable_credential: vc,
        sd_jwt,
        disclosures,
    })
}

pub fn get_metadata() -> Result<IssuerMetadata, IssuerError> {
    debug!("Fetching issuer metadata");
    let public_key_info = crypto::get_public_key_info().map_err(IssuerError::from)?;

    Ok(IssuerMetadata {
        id: "did:example:123".to_string(),
        name: "Example University".to_string(),
        public_key: public_key_info,
    })
}

fn get_credential_type(types: &[String]) -> Result<&str, IssuerError> {
    types
        .iter()
        .find(|&t| t != "VerifiableCredential")
        .map(String::as_str)
        .ok_or_else(|| {
            error!("No specific credential type provided");
            IssuerError::InvalidType("No specific credential type provided".to_string())
        })
}

fn get_schema(credential_type: &str) -> Result<CredentialSchema, IssuerError> {
    schema::get_schema(credential_type).ok_or_else(|| {
        error!("Unsupported credential type: {}", credential_type);
        IssuerError::InvalidType(format!("Unsupported credential type: {}", credential_type))
    })
}

fn create_unsigned_credential(
    request: CredentialRequest,
) -> Result<CredentialResponse, IssuerError> {
    let credential_id = Uuid::new_v4().to_string();
    debug!("Generated credential ID: {}", credential_id);

    Ok(CredentialResponse {
        context: request.context,
        id: Some(format!("http://example.edu/credentials/{}", credential_id)),
        types: request.types,
        issuer: "did:example:123".to_string(), // Issuer側で設定
        issuance_date: Utc::now().to_rfc3339(), // Issuer側で設定
        credential_subject: request.credential_subject,
        proof: None,
        sd_jwt: None,
        disclosures: None,
    })
}

fn sign_and_finalize_credential(
    mut credential: CredentialResponse,
) -> Result<CredentialResponse, IssuerError> {
    let credential_json = serde_json::to_value(&credential)
        .map_err(|e| IssuerError::SerializationError(e.to_string()))?;

    let proof = crypto::sign_json(&credential_json)
        .map_err(|e| IssuerError::SigningError(e.to_string()))?;

    credential.proof = Some(proof);
    info!(
        "Credential signed successfully. ID: {}",
        credential.id.as_ref().unwrap()
    );
    Ok(credential)
}

fn create_sd_jwt(request: &SDJWTCredentialRequest) -> Result<(String, Vec<String>), IssuerError> {
    let mut claims = json!({
        "iss": "did:example:123",  // Issuer側で設定
        "iat": Utc::now().timestamp(),
        "vct": "SDJWTCredential",
        "_sd_alg": "sha-256",
    });

    let mut disclosures = Vec::new();
    let mut sd_claims = Vec::new();

    // Issuer側で選択的開示を決定するロジック
    // そのうち動的に設定できるように改修するかも
    let selective_disclosure_claims = vec!["email", "birthdate"]; // 例として

    for (key, value) in request.credential_subject.as_object().unwrap() {
        if selective_disclosure_claims.contains(&key.as_str()) {
            let salt = sd_jwt::create_salt(&format!("{}:{}", key, value));
            let disclosure = sd_jwt::create_disclosure(&salt, key, value);
            let disclosure_hash = sd_jwt::hash_disclosure(&disclosure);
            sd_claims.push(disclosure_hash);
            disclosures.push(salt); // ソルトのみを保存
        } else {
            claims[key] = value.clone();
        }
    }

    claims["_sd"] = json!(sd_claims);

    let sd_jwt =
        crypto::sign_json(&claims).map_err(|e| IssuerError::SigningError(e.to_string()))?;

    Ok((sd_jwt.as_str().unwrap().to_string(), disclosures))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::issuer::api;
    use crate::models::credential::{CredentialRequest, CredentialResponse, IssuerMetadata};
    use crate::models::sd_jwt::SDJWTCredentialRequest;
    use actix_web::{test, web, App};
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
    use chrono::Utc;
    use serde_json::json;

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

    #[test]
    async fn test_create_sd_jwt_credential() {
        let request = SDJWTCredentialRequest {
            credential_subject: json!({
                "given_name": "Alice",
                "family_name": "Smith",
                "email": "alice@example.com",
                "birthdate": "1990-01-01"
            }),
        };

        let result = create_sd_jwt_credential(request);
        assert!(result.is_ok(), "SD-JWT creation failed: {:?}", result.err());

        let response = result.unwrap();

        assert!(response.sd_jwt.is_some(), "SD-JWT should be present");
        assert!(
            response.disclosures.is_some(),
            "Disclosures should be present"
        );

        let sd_jwt = response.sd_jwt.unwrap();
        let parts: Vec<&str> = sd_jwt.split('.').collect();
        assert_eq!(parts.len(), 3, "SD-JWT should have three parts");

        let payload = URL_SAFE_NO_PAD.decode(parts[1]).unwrap();
        let payload: serde_json::Value = serde_json::from_slice(&payload).unwrap();

        assert!(
            payload.get("iss").is_some(),
            "Issuer claim should be present"
        );
        assert!(
            payload.get("iat").is_some(),
            "Issued At claim should be present"
        );
        assert!(
            payload.get("vct").is_some(),
            "VC Type claim should be present"
        );
        assert!(
            payload.get("_sd_alg").is_some(),
            "SD algorithm claim should be present"
        );

        let sd_claims = payload.get("_sd").unwrap().as_array().unwrap();
        assert_eq!(sd_claims.len(), 2, "There should be 2 SD claims");

        assert!(
            payload.get("given_name").is_some(),
            "Given name should be present in clear"
        );
        assert!(
            payload.get("family_name").is_some(),
            "Family name should be present in clear"
        );
        assert!(
            payload.get("email").is_none(),
            "Email should not be present in clear"
        );
        assert!(
            payload.get("birthdate").is_none(),
            "Birthdate should not be present in clear"
        );

        let disclosures = response.disclosures.unwrap();
        assert_eq!(disclosures.len(), 2, "There should be 2 disclosures");

        for disclosure in disclosures {
            // ソルトのフォーマットをチェック（Base64 URL-safe エンコーディング）
            assert!(
                disclosure
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '-' || c == '_'),
                "Disclosure should only contain Base64 URL-safe characters"
            );

            // ソルトの長さをチェック（16バイトのBase64エンコードは22文字になる）
            assert_eq!(
                disclosure.len(),
                22,
                "Salt should be 22 characters long (16 bytes in Base64)"
            );
        }
    }
}
