use actix_web::{web, HttpResponse, Responder};
use crate::models::credential::{CredentialResponse, PresentationRequest};
use super::holder::Holder;
use std::sync::Arc;

pub async fn store_credential(holder: web::Data<Arc<Holder>>, credential: web::Json<CredentialResponse>) -> impl Responder {
    match holder.store_credential(credential.into_inner()) {
        Ok(id) => HttpResponse::Created().json(serde_json::json!({ "id": id, "status": "stored" })),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_credentials(holder: web::Data<Arc<Holder>>) -> impl Responder {
    match holder.get_credentials() {
        Ok(credentials) => HttpResponse::Ok().json(credentials),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn create_presentation(holder: web::Data<Arc<Holder>>, request: web::Json<PresentationRequest>) -> impl Responder {
    match holder.create_presentation(request.into_inner()) {
        Ok(presentation) => HttpResponse::Ok().json(presentation),
        Err(e) => HttpResponse::BadRequest().body(format!("Failed to create presentation: {}", e)),
    }
}
