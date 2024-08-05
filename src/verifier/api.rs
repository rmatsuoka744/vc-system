use actix_web::{web, HttpResponse, Responder};
use crate::models::credential::{CredentialResponse, VerifiablePresentation};
use crate::verifier::error::VerifierError;
use crate::verifier::verifier;

// エラーメッセージを VerifierError に基づいてマッピング
fn map_verifier_error_to_string(error: &VerifierError) -> String {
    match error {
        VerifierError::MissingProof => "Proof is missing".to_string(),
        VerifierError::UntrustedIssuer => "Untrusted issuer".to_string(),
        VerifierError::InvalidCredentialFormat => "Invalid credential format".to_string(),
        VerifierError::InvalidBase64Encoding => "Invalid base64 encoding in payload".to_string(),
        VerifierError::InvalidJsonPayload => "Invalid JSON in payload".to_string(),
        VerifierError::MissingSdAlgClaim => "Missing _sd_alg claim in SD-JWT".to_string(),
        VerifierError::SignatureVerificationFailed(_) => "Signature verification failed".to_string(),
        VerifierError::InternalError(_) => "Internal server error".to_string(),
        VerifierError::UtilsError(_) => "Utility error".to_string(), // UtilsError もカバー
    }
}

// 認証を検証するエンドポイント
pub async fn verify_credential(credential: web::Json<CredentialResponse>) -> impl Responder {
    match verifier::verify_credential(&credential) {
        Ok(is_valid) => HttpResponse::Ok().json(serde_json::json!({
            "verified": is_valid,
            "errors": if is_valid { Vec::<String>::new() } else { vec!["Invalid credential".to_string()] }
        })),
        Err(e) => {
            let error_message = map_verifier_error_to_string(&e);
            HttpResponse::InternalServerError().body(error_message)
        }
    }
}

// プレゼンテーションを検証するエンドポイント
pub async fn verify_presentation(presentation: web::Json<VerifiablePresentation>) -> impl Responder {
    match verifier::verify_presentation(&presentation) {
        Ok(is_valid) => HttpResponse::Ok().json(serde_json::json!({
            "verified": is_valid,
            "errors": if is_valid { Vec::<String>::new() } else { vec!["Invalid presentation".to_string()] }
        })),
        Err(e) => {
            let error_message = map_verifier_error_to_string(&e);
            HttpResponse::InternalServerError().body(error_message)
        }
    }
}
