mod action;
use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(action::index))
            .route("/again", web::get().to(action::index_two))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
