use axum::{response::IntoResponse, Json};
use serde::Serialize;

use crate::error::Result;

#[derive(Serialize)]
struct HealthResponse {
    version: String,
}

pub async fn handler() -> Result<impl IntoResponse> {
    Ok(Json(HealthResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}
