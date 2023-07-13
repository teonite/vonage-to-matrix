use actix_web::{post};
use log::{info};
use serde::Deserialize;
use serde_querystring_actix::QueryString;
use serde;
use actix_web::web::ServiceConfig;


#[derive(Deserialize)]
pub struct VonageInboundMessage {
    // msisdn: String,
    // to: String,
    // #[serde(alias = "messageId")]
    // message_id: String,
    text: String,
    // #[serde(alias = "type")]
    // message_type: String,
    // keyword: String,
    // #[serde(alias = "api-key")]
    // api_key: String,
    // #[serde(alias = "message-timestamp")]
    // message_timestamp: String,
}

#[post("/api/inbound-message")]
async fn handle_inbound_message(QueryString(message): QueryString<VonageInboundMessage>) -> String {
    info!("Received message {}", message.text);
    format!("Received message {}", message.text)
}

pub fn config_service(config: &mut ServiceConfig) {
    config
        .service(handle_inbound_message);
}
