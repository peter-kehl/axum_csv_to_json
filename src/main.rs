#![allow(incomplete_features)]

use axum::http::StatusCode;
use axum::{
    body::Bytes, extract::ContentLengthLimit, response::IntoResponse, routing::post, Router,
};
use axum_csv_to_json::{
    addresses_to_result_with_csv_crate, addresses_to_result_with_own_csv_parser,
};
use std::net::SocketAddr;
use tokio;

const MAX_CONTENT_LENGTH: u64 = 4 * 1073741824;

pub async fn addresses_with_csv_crate(
    body: ContentLengthLimit<Bytes, MAX_CONTENT_LENGTH>,
) -> impl IntoResponse {
    let result = addresses_to_result_with_csv_crate(&body.0);
    match result {
        Ok(json_string) => Ok(json_string),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn addresses_with_own_csv_parser(
    body: ContentLengthLimit<String, MAX_CONTENT_LENGTH>,
) -> impl IntoResponse {
    let result = addresses_to_result_with_own_csv_parser(body.0);
    match result {
        Ok(json_string) => Ok(json_string),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tokio::main]
async fn main() {
    // let app = Router::new().route("/addresses", post(addresses_with_csv_crate));
    let app = Router::new().route("/addresses", post(addresses_with_own_csv_parser));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
