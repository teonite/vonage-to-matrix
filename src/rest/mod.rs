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

#[post("/api/inbound-message")]
async fn handle_inbound_message(web::Form(form): web::Form<VonageInboundMessage>) -> HttpResponse {
    info!("Received message {}", form.text);
    HttpResponse::Ok().finish()
}

pub fn config_service(config: &mut ServiceConfig) {
    config
        .service(handle_inbound_message);
}
