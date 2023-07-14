use actix_web::{test, App};
use vonage_to_matrix::rest::{config_service, VonageInboundMessage};


#[actix_web::test]
async fn test_handle_inbound_message() {
    let app = test::init_service(
        App::new()
            .configure(config_service),
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
        message_timestamp: String::from("2023-07-12+15:30:47")
    };

    let req = test::TestRequest::post().uri("/api/inbound-message").set_form(form).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
