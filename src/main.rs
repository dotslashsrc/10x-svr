
use actix_web::{web, App, HttpServer};

mod bitbucket;
mod gpt;
mod diff;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    eprintln!("Starting server at :8080...");
    HttpServer::new(|| App::new()
            .route("/bitbucket", web::post().to(bitbucket::webhook))

            .route("/diff", web::post().to(diff::http_handler))
            )
        .bind(("localhost", 8080))?
        .run()
        .await
}
