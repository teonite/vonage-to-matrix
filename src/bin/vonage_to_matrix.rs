use std::net::{IpAddr, Ipv4Addr};

use actix_web::{middleware::Logger, App, HttpServer};
use vonage_to_matrix::rest::{config_app_data, config_service};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .configure(config_service)
            .configure(config_app_data)
            .wrap(Logger::default())
    })
    .bind((IpAddr::V4(Ipv4Addr::UNSPECIFIED), 8080))?
    .run()
    .await
}
