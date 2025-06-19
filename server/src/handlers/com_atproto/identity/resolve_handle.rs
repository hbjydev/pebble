use anyhow::anyhow;
use atrium_api::types::string::{Did, Handle};
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use hickory_resolver::{config::ResolverConfig, name_server::TokioConnectionProvider, Resolver};
use serde::{Deserialize, Serialize};

use crate::{error::Result, utils::{DnsResolver, HandleResolver}};

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
) -> Result<impl IntoResponse> {
    if query.handle.clone() == "" {
        return Err(anyhow!("Handle must be supplied in the request.").into());
    }

    let handle = Handle::new(query.handle.clone())
        .map_err(|_| anyhow!("invalid handle"))?;

    let resolver = DnsResolver::new();

    if let Some(did) = resolver.resolve_handle(handle).await {
        return Ok(
            (
                StatusCode::OK,
                Json(ResolveHandleResponse {
                    did,
                })
            )
        );
    }

    Err(anyhow::anyhow!("failed to find did for handle").into())
}
