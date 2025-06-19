use axum::{extract::FromRef, routing::get, Router};

use crate::config::Config;

mod handle_root;
mod well_known {
    pub mod did;
}

mod com_atproto {
    pub mod identity {
        pub mod resolve_handle;
    }
}

#[derive(Clone)]
struct AppState {
    pub config: Config,
}

impl FromRef<AppState> for Config {
    fn from_ref(input: &AppState) -> Self {
        input.config.clone()
    }
}

pub fn make_router(config: Config) -> Router {
    Router::new()
        .route("/", get(handle_root::handler))
        .route("/.well-known/did.json", get(well_known::did::handler))

        // public endpoints
        .route("/xrpc/com.atproto.identity.resolveHandle", get(com_atproto::identity::resolve_handle::handler))

        .with_state(AppState {
            config,
        })
}
