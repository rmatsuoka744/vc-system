use actix_web::{web, HttpResponse, Responder};
use crate::models::credential::CredentialRequest;
use super::issuer;

pub async fn issue_credential(credential_request: web::Json<CredentialRequest>) -> impl Responder {
    match issuer::create_credential(credential_request.into_inner()) {
        Ok(credential) => HttpResponse::Created().json(credential),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create credential"),
    }
}

pub async fn get_issuer_metadata() -> impl Responder {
    match issuer::get_metadata() {
        Ok(metadata) => HttpResponse::Ok().json(metadata),
        Err(_) => HttpResponse::InternalServerError().json("Failed to retrieve issuer metadata"),
    }
}
