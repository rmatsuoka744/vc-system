use actix_web::{web, App, HttpServer};
use std::sync::Arc;

mod holder;
mod issuer;
mod verifier;
mod models;
mod utils;

use holder::storage::MemoryStorage;
use holder::holder::Holder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
                    .route("/credentials", web::post().to(holder::api::store_credential))
                    .route("/credentials", web::get().to(holder::api::get_credentials))
                    .route("/presentations", web::post().to(holder::api::create_presentation))
            )
            // Issuer のルートを設定
            .service(
                web::scope("/issuer")
                    .route("/credentials", web::post().to(issuer::api::issue_credential))
                    .route("/metadata", web::get().to(issuer::api::get_issuer_metadata))
            )
            // Verifier のルートを設定
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
