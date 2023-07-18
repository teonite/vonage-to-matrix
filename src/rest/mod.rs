use std::collections::HashMap;
use std::fs;
use actix_web::{HttpResponse, post, web};
use log::{info};
use vonage::{VonageInboundMessage, VonageInboundCall};
use hookshot::{send_text_message};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct HookshotConfig {
    url: String,
}

#[derive(Deserialize)]
pub struct VonageConfig {
    labels: HashMap<String, String>
}

#[derive(Deserialize)]
pub struct Config {
    hookshot: HookshotConfig,
    vonage: VonageConfig,
}

#[post("/api/inbound-message")]
async fn handle_inbound_message(web::Form(form): web::Form<VonageInboundMessage>, config: web::Data<Config>) -> HttpResponse {
    info!("Received message from {}", form.msisdn);
    let label = config.vonage.labels.get(form.to.as_str()).unwrap();
    let message = format!("Received text message from {} (to number: {} [{}]): {}", form.msisdn, label, form.to, form.text);
    handle_vonage_event(config.hookshot.url.clone(), message).await
}

#[post("/api/inbound-call")]
async fn handle_inbound_call(web::Form(form): web::Form<VonageInboundCall>, config: web::Data<Config>) -> HttpResponse {
    info!("Received call from {}", form.from);
    let message = format!("Received call from {}", form.from);
    handle_vonage_event(config.hookshot.url.clone(), message).await
}

async fn handle_vonage_event(hookshot_webhook_url: String, message: String) -> HttpResponse {
    let result = send_text_message(hookshot_webhook_url, message).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::ServiceUnavailable().finish(),
    }
}

pub fn config_service(config: &mut web::ServiceConfig) {
    config
        .service(handle_inbound_message)
        .service(handle_inbound_call);
}

fn read_app_config_from_file() -> Config {
    let filename = "config.toml";
    let contents = fs::read_to_string(filename).unwrap();
    toml::from_str(&contents).unwrap()
}

pub fn config_app_data(config: &mut web::ServiceConfig) {
    let app_config: Config = read_app_config_from_file();

    config
        .app_data(web::Data::new(app_config));
}