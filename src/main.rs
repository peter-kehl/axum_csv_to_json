#![allow(incomplete_features)]

use axum::http::StatusCode;
use axum::{
    body::Bytes, extract::ContentLengthLimit, response::IntoResponse, routing::post, Router,
};
use axum_csv_to_json::{addresses_to_result, MAX_CONTENT_LENGTH};
use std::net::SocketAddr;
use tokio;

// We could use String instead of Bytes, but we need a byte slice for `from_reader`
pub async fn addresses(body: ContentLengthLimit<Bytes, MAX_CONTENT_LENGTH>) -> impl IntoResponse {
    println!("addresses(body) called");
    let result = addresses_to_result(body).await;
    println!("addresses(body) got a result");
    match result {
        Ok(json_string) => Ok(json_string),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/addresses", post(addresses));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
