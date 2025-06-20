use atrium_api::types::string::{Did, Handle};
use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Json};
use fred::prelude::KeysInterface;
use serde::{Deserialize, Serialize};

use crate::{config::Config, error::{AppError, Result}, utils::resolve_handle};

#[derive(Deserialize)]
pub struct ResolveHandleQuery {
    handle: String,
}

#[derive(Serialize)]
struct ResolveHandleResponse {
    did: Did,
}

#[axum::debug_handler]
pub async fn handler(
    query: Query<ResolveHandleQuery>,
    State(cfg): State<Config>,
) -> Result<impl IntoResponse> {
    if query.handle.clone() == "" {
        return Err(AppError::InvalidHandle);
    }

    let handle = Handle::new(query.handle.clone())
        .map_err(|_| AppError::InvalidHandle)?;

    let cache_key = format!("hr:{}", handle.to_string());

    let client = cfg.redis.cache();
    if let Some(did_str) = client.get::<Option<String>, String>(cache_key.clone()).await? {
        if let Ok(did) = Did::new(did_str) {
            return Ok(
                (
                    StatusCode::OK,
                    Json(ResolveHandleResponse {
                        did,
                    })
                )
            );
        };
    }

    if let Some(did) = resolve_handle(handle).await {
        client.set::<(), String, String>(
            cache_key,
            did.to_string(),
            Some(fred::types::Expiration::EX(300)),
            None,
            false,
        ).await?;

        return Ok(
            (
                StatusCode::OK,
                Json(ResolveHandleResponse {
                    did,
                })
            )
        );
    }

    Err(AppError::InvalidHandle)
}
