use std::collections::HashMap;
use reqwest::{Error, Response};
use log::{info};

pub async fn send_text_message(webhook_url: String, text: String) -> Result<Response, Error> {
    info!("Sending message '{}' to {}", text, webhook_url);

    let mut map = HashMap::new();
    map.insert("text", text);

    #[cfg(not(test))]
    let client = reqwest::Client::new();

    #[cfg(test)]
    let client = reqwest_mock::Client::new();

    client.post(webhook_url)
        .json(&map)
        .send()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
}
