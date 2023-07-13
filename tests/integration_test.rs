use actix_web::{test, App};
use vonage_to_matrix::rest::{config_service};


#[actix_web::test]
async fn test_handle_inbound_message() {
    let app = test::init_service(
        App::new()
            .configure(config_service),
    )
    .await;

    let req = test::TestRequest::post().uri("/api/inbound-message?text=Testing123").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
