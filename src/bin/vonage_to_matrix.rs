use actix_web::{App, HttpServer};

use vonage_to_matrix::{rest::{config_service, config_app_data}};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .configure(config_service)
            .configure(config_app_data)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
