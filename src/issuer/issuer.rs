use crate::issuer::error::IssuerError;
use crate::issuer::schema;
use crate::models::credential::{CredentialRequest, CredentialResponse, IssuerMetadata};
use crate::models::schema::CredentialSchema; // この行を追加
use crate::models::sd_jwt::{SDJWTCredentialRequest, SDJWTCredentialResponse};
use crate::utils::{crypto, sd_jwt};
use chrono::Utc;
use log::{debug, error, info};
use serde_json::{json, Value};
use uuid::Uuid;

pub fn create_credential(request: CredentialRequest) -> Result<CredentialResponse, IssuerError> {
    info!("Received credential request: {:?}", request);

    let credential_type = get_credential_type(&request.types)?;
    let schema = get_schema(credential_type)?;

    schema::validate_credential_subject(&request.credential_subject, &schema)?;
    debug!("Credential subject validated successfully");

    let credential = create_unsigned_credential(request)?;
    sign_and_finalize_credential(credential)
}

pub fn create_sd_jwt_credential(
    request: SDJWTCredentialRequest,
) -> Result<CredentialResponse, IssuerError> {
    info!("Creating SD-JWT credential");

    let (claims, disclosures) = create_sd_jwt_claims(&request)?;
    let sd_jwt = sign_sd_jwt_claims(claims)?;

    // 通常のVC部分を作成
    let vc_request = CredentialRequest {
        context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
        types: vec![
            "VerifiableCredential".to_string(),
            "SDJWTCredential".to_string(),
        ],
        issuer: "did:example:123".to_string(),
        issuance_date: Utc::now().to_rfc3339(),
        credential_subject: request.credential_subject.clone(),
    };
    let mut vc = create_credential(vc_request)?;

    // VCのcredential_subjectから選択的開示属性を削除
    for attr in &disclosures {
        let parts: Vec<&str> = attr.splitn(3, '.').collect();
        if parts.len() == 3 {
            vc.credential_subject
                .as_object_mut()
                .unwrap()
                .remove(parts[1]);
        }
    }

    // SD-JWT情報をVCに追加
    vc.sd_jwt = Some(sd_jwt);
    vc.disclosures = Some(disclosures);

    Ok(vc)
}

pub fn create_sd_jwt_vc(
    request: SDJWTCredentialRequest,
) -> Result<SDJWTCredentialResponse, IssuerError> {
    let vc = create_sd_jwt_credential(request)?;

    Ok(SDJWTCredentialResponse {
        verifiable_credential: vc.clone(),
        sd_jwt: vc.sd_jwt.unwrap_or_default(),
        disclosures: vc.disclosures.unwrap_or_default(),
    })
}

pub fn get_metadata() -> Result<IssuerMetadata, IssuerError> {
    debug!("Fetching issuer metadata");
    let public_key_info = crypto::get_public_key_info().map_err(|e| IssuerError::CryptoError(e))?;

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
        issuer: request.issuer,
        issuance_date: Utc::now().to_rfc3339(),
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

    let proof = crypto::sign_json(&credential_json).map_err(|e| IssuerError::SigningError(e))?;

    credential.proof = Some(proof);
    info!(
        "Credential signed successfully. ID: {}",
        credential.id.as_ref().unwrap()
    );
    Ok(credential)
}

fn create_sd_jwt_claims(
    request: &SDJWTCredentialRequest,
) -> Result<(Value, Vec<String>), IssuerError> {
    let mut claims = json!({
        "iss": "did:example:123",
        "iat": Utc::now().timestamp(),
        "_sd_alg": "sha-256",
    });

    let mut disclosures = Vec::new();
    let mut sd_claims = json!({});

    for claim_name in &request.selective_disclosure {
        if let Some(claim_value) = request.credential_subject.get(claim_name) {
            let salt = sd_jwt::create_salt();
            let disclosure = sd_jwt::create_disclosure(&salt, claim_name, claim_value);
            let disclosure_hash = sd_jwt::hash_disclosure(&disclosure);

            sd_claims[claim_name] = json!(disclosure_hash);
            disclosures.push(disclosure);
        }
    }

    claims["_sd"] = sd_claims;

    for (key, value) in request.credential_subject.as_object().unwrap() {
        if !request.selective_disclosure.contains(key) {
            claims[key] = value.clone();
        }
    }

    Ok((claims, disclosures))
}

fn sign_sd_jwt_claims(claims: Value) -> Result<String, IssuerError> {
    debug!("Claims before signing: {:?}", claims);
    let sd_jwt = crypto::sign_json(&claims)?;
    match sd_jwt {
        Value::String(jwt) => {
            debug!("Signed SD-JWT: {}", jwt);
            Ok(jwt)
        }
        _ => Err(IssuerError::SigningError(
            "Invalid SD-JWT format".to_string(),
        )),
    }
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
        // テスト用のリクエストを作成
        let request = SDJWTCredentialRequest {
            credential_subject: json!({
                "given_name": "Alice",
                "family_name": "Smith",
                "email": "alice@example.com",
                "birthdate": "1990-01-01"
            }),
            selective_disclosure: vec!["email".to_string(), "birthdate".to_string()],
        };

        // SD-JWTクレデンシャルを生成
        let result = create_sd_jwt_credential(request);
        assert!(result.is_ok(), "SD-JWT creation failed: {:?}", result.err());
        assert!(result.is_ok(), "SD-JWT creation should succeed");

        let response = result.unwrap();

        println!("SD-JWT: {:?}", response.sd_jwt);
        println!("Disclosures: {:?}", response.disclosures);

        // SD-JWTの構造を検証
        let sd_jwt = response.sd_jwt.expect("SD-JWT should be present");
        let parts: Vec<&str> = sd_jwt.split('.').collect();
        assert_eq!(parts.len(), 3, "SD-JWT should have three parts");

        // ペイロードをデコードして検証
        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(parts[1])
            .unwrap();
        let payload: serde_json::Value = serde_json::from_slice(&payload).unwrap();
        println!("Decoded SD-JWT payload: {:?}", payload);

        // 必須クレームの存在を確認
        assert!(
            payload.get("iss").is_some(),
            "Issuer claim should be present"
        );
        assert!(
            payload.get("iat").is_some(),
            "Issued At claim should be present"
        );
        assert!(
            payload.get("_sd_alg").is_some(),
            "SD algorithm claim should be present"
        );

        // 選択的開示クレームの検証
        let sd_claims = payload.get("_sd").unwrap().as_object().unwrap();
        assert!(
            sd_claims.contains_key("email"),
            "Email should be in _sd claims"
        );
        assert!(
            sd_claims.contains_key("birthdate"),
            "Birthdate should be in _sd claims"
        );

        // 非選択的開示クレームの検証
        assert_eq!(
            payload["given_name"], "Alice",
            "Given name should be present in clear"
        );
        assert_eq!(
            payload["family_name"], "Smith",
            "Family name should be present in clear"
        );

        // Disclosuresの数を検証
        let disclosures = response.disclosures.expect("Disclosures should be present");
        assert_eq!(disclosures.len(), 2, "There should be 2 disclosures");

        // Disclosuresの形式を検証
        for (index, disclosure) in disclosures.iter().enumerate() {
            println!("Checking disclosure {}: {:?}", index, disclosure);
            let parts: Vec<&str> = disclosure.splitn(3, '.').collect();
            assert_eq!(parts.len(), 3, "Each disclosure should have three parts");
            assert!(!parts[0].is_empty(), "Salt should not be empty");
            assert!(!parts[1].is_empty(), "Claim name should not be empty");
            assert!(!parts[2].is_empty(), "Claim value should not be empty");
        }
    }
}
