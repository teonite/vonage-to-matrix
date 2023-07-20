use actix_web::{post, web, HttpResponse};
use hookshot::send_text_message;
use log::{info, warn};
use serde::Deserialize;
use std::{collections::HashMap, fs};
use vonage::{VonageInboundCall, VonageInboundMessage};

#[derive(Deserialize)]
pub struct HookshotConfig {
    pub url: String,
}

#[derive(Deserialize)]
pub struct VonageConfig {
    pub labels: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub hookshot: HookshotConfig,
    pub vonage: VonageConfig,
}

#[post("/api/inbound-message")]
async fn handle_inbound_message(
    web::Form(form): web::Form<VonageInboundMessage>,
    config: web::Data<Config>,
) -> HttpResponse {
    info!("Received message from {}", form.msisdn);
    let vonage_label = get_vonage_label(&form.to, &config.vonage);
    let message = format!(
        "Received text message from {} (to number: {} [{}]): {}",
        form.msisdn, vonage_label, form.to, form.text
    );
    handle_vonage_event(&config.hookshot.url, message).await
}

#[post("/api/inbound-call")]
async fn handle_inbound_call(
    web::Form(form): web::Form<VonageInboundCall>,
    config: web::Data<Config>,
) -> HttpResponse {
    info!("Received call from {}", form.from);
    let vonage_label = get_vonage_label(&form.to, &config.vonage);
    let message = format!(
        "Received call from {} (to number: {} [{}])",
        form.from, vonage_label, form.to
    );
    handle_vonage_event(&config.hookshot.url, message).await
}

async fn handle_vonage_event(hookshot_webhook_url: &str, message: String) -> HttpResponse {
    let result = send_text_message(hookshot_webhook_url, message).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::ServiceUnavailable().finish(),
    }
}

fn get_vonage_label(to_number: &str, config: &VonageConfig) -> String {
    if let Some(l) = config.labels.get(to_number) {
        l.clone()
    } else {
        warn!("Returning default label for number {to_number}");
        String::from("Unlabeled")
    }
}

pub fn config_service(config: &mut web::ServiceConfig) {
    config
        .service(handle_inbound_message)
        .service(handle_inbound_call);
}

fn read_app_config_from_file() -> Config {
    let filename = "config.toml";
    let contents = fs::read_to_string(filename).expect("Error while reading config file");
    toml::from_str(&contents).expect("Error while parsing config file")
}

pub fn config_app_data(config: &mut web::ServiceConfig) {
    let app_config: Config = read_app_config_from_file();

    config.app_data(web::Data::new(app_config));
}
