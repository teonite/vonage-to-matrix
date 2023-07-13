use actix_web::{App, HttpServer};

use vonage_to_matrix::{rest::config_service};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .configure(config_service)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
