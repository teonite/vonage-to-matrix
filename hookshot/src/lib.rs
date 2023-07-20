use log::info;
use reqwest::{Error, Response};
use std::collections::HashMap;

pub async fn send_text_message(webhook_url: &str, text: String) -> Result<Response, Error> {
    info!("Sending message '{text}' to {webhook_url}");

    let mut map = HashMap::new();
    map.insert("text", text);

    #[cfg(not(test))]
    let client = reqwest::Client::new();

    #[cfg(test)]
    let client = reqwest_mock::Client::new();

    client.post(webhook_url).json(&map).send().await
}
