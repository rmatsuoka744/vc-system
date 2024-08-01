use actix_web::{App, HttpServer, web};

mod issuer;
mod holder;
mod verifier;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/issuer")
                    .route("/credentials", web::post().to(issuer::api::issue_credential))
                    .route("/metadata", web::get().to(issuer::api::get_issuer_metadata))
            )
            .service(
                web::scope("/holder")
                    .route("/credentials", web::post().to(holder::api::store_credential))
                    .route("/credentials", web::get().to(holder::api::get_credentials))
                    .route("/presentations", web::post().to(holder::api::create_presentation))
            )
            .service(
                web::scope("/verifier")
                    .route("/verify/credential", web::post().to(verifier::api::verify_credential))
                    .route("/verify/presentation", web::post().to(verifier::api::verify_presentation))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
