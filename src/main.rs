use actix_web::{web, App, HttpServer};
use env_logger::Env;
use std::sync::Arc;

mod holder;
mod issuer;
mod models;
mod utils;
mod verifier;

use holder::holder::Holder;
use holder::storage::MemoryStorage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    // ストレージの初期化
    let storage = Arc::new(MemoryStorage::new());

    // Holder インスタンスの作成
    let holder = Arc::new(Holder::new(storage));

    // サーバーの設定と起動
    HttpServer::new(move || {
        App::new()
            // Holder のデータを共有データとして追加
            .app_data(web::Data::new(holder.clone()))
            // Holder のルートを設定
            .service(
                web::scope("/holder")
                    .route(
                        "/credentials",
                        web::post().to(holder::api::store_credential),
                    )
                    .route("/credentials", web::get().to(holder::api::get_credentials))
                    .route(
                        "/presentations",
                        web::post().to(holder::api::create_presentation),
                    ),
            )
            // Issuer のルートを設定
            .service(
                web::scope("/issuer")
                    .route(
                        "/credentials",
                        web::post().to(issuer::api::issue_credential),
                    )
                    .route("/metadata", web::get().to(issuer::api::get_issuer_metadata))
                    .route(
                        "/sd-jwt-credentials",
                        web::post().to(issuer::api::issue_sd_jwt_credential),
                    ),
            )
            // Verifier のルートを設定
            .service(
                web::scope("/verifier")
                    .route(
                        "/credentials",
                        web::post().to(verifier::api::verify_credential),
                    )
                    .route(
                        "/presentations",
                        web::post().to(verifier::api::verify_presentation),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

use env_logger;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
}
