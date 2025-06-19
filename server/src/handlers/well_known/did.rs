use atrium_api::types::string::Did;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::config::Config;

#[derive(Serialize)]
struct WellKnownDidResponseServiceDef {
    pub id: String,
    #[serde(rename = "type")]
    pub service_type: String,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
}

#[derive(Serialize)]
struct WellKnownDidResponse {
    pub context: Vec<String>,
    pub id: Did,
    pub services: Vec<WellKnownDidResponseServiceDef>,
}

pub async fn handler(
    State(config): State<Config>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(WellKnownDidResponse {
        context: vec![String::from("https://www.w3.org/ns/did/v1")],
        id: config.atproto.did,
        services: vec![
            WellKnownDidResponseServiceDef{
                id: String::from("#atproto_pds"),
                service_type: String::from("AtprotoPersonalDataServer"),
                service_endpoint: format!("https://{}", config.atproto.hostname),
            },
        ],
    }))
}
