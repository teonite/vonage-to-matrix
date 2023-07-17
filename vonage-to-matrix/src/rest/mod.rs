use actix_web::{HttpResponse, post, web};
use log::{info};
use actix_web::web::ServiceConfig;
use vonage::{VonageInboundMessage, VonageInboundCall};


#[post("/api/inbound-message")]
async fn handle_inbound_message(web::Form(form): web::Form<VonageInboundMessage>) -> HttpResponse {
    info!("Received message {}", form.text);
    HttpResponse::Ok().finish()
}

#[post("/api/inbound-call")]
async fn handle_inbound_call(web::Form(form): web::Form<VonageInboundCall>) -> HttpResponse {
    info!("Received call from {}", form.from);
    HttpResponse::Ok().finish()
}

pub fn config_service(config: &mut ServiceConfig) {
    config
        .service(handle_inbound_message)
        .service(handle_inbound_call);
}
