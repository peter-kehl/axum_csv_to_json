//#![allow(incomplete_features)]

use axum::http::StatusCode;
use axum::{
    body::Bytes, extract::ContentLengthLimit, response::IntoResponse, routing::post, Router,
};
// @TODO Why doesn't `crate` work here? As in: use crate::addresses_to_result_with_csv_crate
use axum_csv_to_json::logger::Logger;
use axum_csv_to_json::{
    addresses_to_result_with_csv_crate, addresses_to_result_with_own_csv_parser,
};
use std::net::SocketAddr;
use tokio;

const MAX_CONTENT_LENGTH: u64 = 4 * 1073741824; // 4GB

pub async fn addresses_with_csv_crate(
    logger: Logger,
    body: ContentLengthLimit<Bytes, MAX_CONTENT_LENGTH>,
) -> impl IntoResponse {
    let result = addresses_to_result_with_csv_crate(logger, &body.0).await;
    match result {
        Ok(json_string) => Ok(json_string),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn addresses_with_own_csv_parser(
    logger: Logger,
    body: ContentLengthLimit<String, MAX_CONTENT_LENGTH>,
) -> impl IntoResponse {
    let result = addresses_to_result_with_own_csv_parser(logger, body.0).await;
    match result {
        Ok(json_string) => Ok(json_string),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tokio::main]
async fn main() {
    let logger = Logger::new();
    assert!(logger
        .info("Instantiating (HTTP) Router struct.")
        .await
        .is_ok());

    // compile-time choice of implementation. By having this if/false we copile-time check both function signatures.
    let app = if false {
        Router::new().route(
            "/addresses",
            post(|body|
            // have to create a new Logger, since it's not Clone
            addresses_with_own_csv_parser(Logger::new(), body)),
        )
    } else {
        Router::new().route(
            "/addresses",
            post(|body|
        // have to create a new Logger, since it's not Clone
         addresses_with_csv_crate(Logger::new(), body)),
        )
    };

    assert!(logger.info("Starting (HTTP) Router.").await.is_ok());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
