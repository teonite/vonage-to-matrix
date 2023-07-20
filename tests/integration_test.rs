use actix_web::{test, web, App};
use std::collections::HashMap;
use vonage::{VonageInboundCall, VonageInboundMessage};
use vonage_to_matrix::rest::{config_service, Config, HookshotConfig, VonageConfig};

fn config_app_data(config: &mut web::ServiceConfig) {
    let hookshot_config = HookshotConfig {
        url: String::from("https://example.com"),
    };

    let vonage_config = VonageConfig {
        labels: HashMap::from([
            (String::from("48780909100"), String::from("Poland")),
            (String::from("16692609100"), String::from("United States")),
        ]),
    };

    let app_config = Config {
        hookshot: hookshot_config,
        vonage: vonage_config,
    };

    config.app_data(web::Data::new(app_config));
}

#[actix_web::test]
async fn test_handle_inbound_message() {
    let app = test::init_service(
        App::new()
            .configure(config_service)
            .configure(config_app_data),
    )
    .await;

    let form = VonageInboundMessage {
        msisdn: String::from("48111111111"),
        to: String::from("48222222222"),
        message_id: String::from("2A000000160BBF61"),
        text: String::from("Test"),
        message_type: String::from("text"),
        keyword: String::from("TEST"),
        api_key: String::from("56376bb4"),
        message_timestamp: String::from("2023-07-12+15:30:47"),
    };

    let req = test::TestRequest::post()
        .uri("/api/inbound-message")
        .set_form(form)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_handle_inbound_call() {
    let app = test::init_service(
        App::new()
            .configure(config_service)
            .configure(config_app_data),
    )
    .await;

    let form = VonageInboundCall {
        call_id: String::from("2adb66c0b8f71835ba2a570956063b2c"),
        status: String::from("ok"),
        call_direction: String::from("in"),
        to: String::from("48111111111"),
        from: String::from("48222222222"),
        call_request: String::from("2023-07-14+14%3A27%3A23"),
        network_code: String::from("26006"),
        call_duration: String::from("3"),
        call_start: String::from("2023-07-14+14%3A27%3A27"),
        call_end: String::from("2023-07-14+14%3A27%3A30"),
        call_price: String::from("0.00036000"),
        call_rate: String::from("0.00720000"),
    };

    let req = test::TestRequest::post()
        .uri("/api/inbound-call")
        .set_form(form)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
