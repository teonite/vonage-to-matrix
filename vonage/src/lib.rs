use serde::{Deserialize, Serialize};
use serde;

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

#[cfg(test)]
mod tests {
    use super::*;
}
