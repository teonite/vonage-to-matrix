use std::collections::HashMap;
use reqwest::{Error, Response};
use log::{info};

pub async fn send_text_message(webhook_url: &str, text: String) -> Result<Response, Error> {
    info!("Sending message {} to {}", text, webhook_url);

    let mut map = HashMap::new();
    map.insert("text", text);

    let client = reqwest::Client::new();

    client.post(webhook_url)
        .json(&map)
        .send()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
}
