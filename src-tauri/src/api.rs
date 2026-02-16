use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

// 👉 dein bestehender Pfad
use crate::events::frontend::settings::set_brightness;

#[derive(Deserialize)]
struct BrightnessParams {
    brightness: u8,
}

async fn set_brightness_handler(
    Query(params): Query<BrightnessParams>,
) -> impl IntoResponse {
    match set_brightness(params.brightness).await {
        Ok(_) => (StatusCode::OK, "ok").into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).0.into_response(),
    }
}

pub async fn start_api_server() {
    let app = Router::new()
        .route("/set_brightness", post(set_brightness_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind API socket");

    axum::serve(listener, app)
        .await
        .expect("API server crashed");
}
