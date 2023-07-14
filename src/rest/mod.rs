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
    #[serde(alias = "call-direction")]
    pub call_direction: String,
    pub to: String,
    pub from: String,
    #[serde(alias = "call-request")]
    pub call_request: String,
    #[serde(alias = "network-code")]
    pub network_code: String,
    #[serde(alias = "call-duration")]
    pub call_duration: String,
    #[serde(alias = "call-start")]
    pub call_start: String,
    #[serde(alias = "call-end")]
    pub call_end: String,
    #[serde(alias = "call-price")]
    pub call_price: String,
    #[serde(alias = "call-rate")]
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
