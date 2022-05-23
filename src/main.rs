use axum::{
    body::Bytes, extract::ContentLengthLimit, response::IntoResponse, routing::post, Router,
};
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Result as SerdeResult;
use std::net::SocketAddr;
use tokio;

const MAX_CONTENT_LENGTH: u64 = 4 * 1073741824;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
enum AddressType {
    appt,
    house,
    suite,
}

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    reference: String,
    address_type: AddressType,
    suite_number: Option<String>,
    street_number: i32, //TODO check address standards
    street: String,
    city: String,
    state: String,
    postcode: String, //to preserve any leading zeros (if allowed - TODO: check address standards)
}

// We could use String instead of Bytes, but we need a byte slice for `from_reader`
pub async fn addresses(body: ContentLengthLimit<Bytes, MAX_CONTENT_LENGTH>) -> impl IntoResponse {
    let body_struct = body.0;
    let bytes: &[u8] = &body_struct;
    let mut reader = Reader::from_reader(bytes);
    todo!()
    //Json(item.message)
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
