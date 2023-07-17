use actix_web::{HttpResponse, post, web};
use log::{info};
use serde::{Deserialize, Serialize};
use serde;
use actix_web::web::ServiceConfig;


#[derive(Deserialize, Serialize)]
pub struct VonageInboundMessage {
    pub msisdn: String,
    pub to: String,
    #[serde(alias = "messageId")]
    pub message_id: String,
    pub text: String,
    #[serde(alias = "type")]
    pub message_type: String,
    pub keyword: String,
    #[serde(alias = "api-key")]
    pub api_key: String,
    #[serde(alias = "message-timestamp")]
    pub message_timestamp: String,
}

#[derive(Deserialize, Serialize)]
pub struct VonageInboundCall {
    pub call_id: String,
    pub status: String,
    pub call_direction: String,
    pub to: String,
    pub from: String,
    pub call_request: String,
    pub network_code: String,
    pub call_duration: String,
    pub call_start: String,
    pub call_end: String,
    pub call_price: String,
    pub call_rate: String,
}

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
