use actix_web::{HttpResponse, post, web};
use log::{info};
use actix_web::web::ServiceConfig;
use vonage::{VonageInboundMessage, VonageInboundCall};
use hookshot::{send_text_message};


const HOOKSHOT_WEBHOOK_URL: &str = "https://hookshot.teonite.net/webhook/141d7f48-862e-42a8-a1e7-efab3a37ed09";


#[post("/api/inbound-message")]
async fn handle_inbound_message(web::Form(form): web::Form<VonageInboundMessage>) -> HttpResponse {
    info!("Received message {}", form.text);
    let result = send_text_message(HOOKSHOT_WEBHOOK_URL, format!("Received text message from {}: {}", form.msisdn, form.text)).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::ServiceUnavailable().finish(),
    }

}

#[post("/api/inbound-call")]
async fn handle_inbound_call(web::Form(form): web::Form<VonageInboundCall>) -> HttpResponse {
    info!("Received call from {}", form.from);
    let result = send_text_message(HOOKSHOT_WEBHOOK_URL, format!("Received call from {}", form.from)).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::ServiceUnavailable().finish(),
    }
}

pub fn config_service(config: &mut ServiceConfig) {
    config
        .service(handle_inbound_message)
        .service(handle_inbound_call);
}
