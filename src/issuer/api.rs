use super::issuer;
use crate::models::credential::CredentialRequest;
use crate::models::sd_jwt::SDJWTCredentialRequest;
use actix_web::{web, HttpResponse, Responder};
use log::error;

pub async fn issue_credential(request: web::Json<CredentialRequest>) -> impl Responder {
    match issuer::create_credential(request.into_inner()) {
        Ok(credential) => HttpResponse::Ok().json(credential),
        Err(e) => {
            error!("Failed to issue credential: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Failed to issue credential: {:?}", e))
        }
    }
}

pub async fn get_issuer_metadata() -> impl Responder {
    match issuer::get_metadata() {
        Ok(metadata) => HttpResponse::Ok().json(metadata),
        Err(_) => HttpResponse::InternalServerError().json("Failed to retrieve issuer metadata"),
    }
}

pub async fn issue_sd_jwt_credential(request: web::Json<SDJWTCredentialRequest>) -> impl Responder {
    match issuer::create_sd_jwt_vc(request.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
