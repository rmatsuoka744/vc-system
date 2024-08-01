use actix_web::{web, HttpResponse, Responder};
use crate::models::credential::{CredentialResponse, VerifiablePresentation};
use super::verifier;

pub async fn verify_credential(credential: web::Json<CredentialResponse>) -> impl Responder {
    match verifier::verify_credential(&credential) {
        Ok(is_valid) => HttpResponse::Ok().json(serde_json::json!({
            "verified": is_valid,
            "errors": if is_valid { Vec::<String>::new() } else { vec!["Invalid credential".to_string()] }
        })),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn verify_presentation(presentation: web::Json<VerifiablePresentation>) -> impl Responder {
    match verifier::verify_presentation(&presentation) {
        Ok(is_valid) => HttpResponse::Ok().json(serde_json::json!({
            "verified": is_valid,
            "errors": if is_valid { Vec::<String>::new() } else { vec!["Invalid presentation".to_string()] }
        })),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
