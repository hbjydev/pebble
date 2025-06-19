pub mod args;

mod error;
mod handlers;
mod utils;

mod config;
use config::*;
use handlers::make_router;

#[derive(Debug, Clone)]
pub struct PebbleServer {
    config: Config,
}

impl PebbleServer {
    pub fn new(args: args::PebbleServerArgs) -> Self {
        let config = Config {
            server: ServerConfig {
                bind_addr: args.bind_addr,
            },

            atproto: AtpConfig {
                did: args.did,
                contact_email: args.contact_email,
                relays: args.relays,
                hostname: args.hostname,
            },

            db: DbConfig {
                name: args.db_name,
                user: args.db_user,
                password: args.db_password,
                host: args.db_host,
                port: args.db_port,
            },

            s3: S3Config {
                bucket_name: args.s3_bucket_name,
            },
        };

        Self { config }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let router = make_router(self.config.clone());
        let listener = tokio::net::TcpListener::bind(self.config.server.bind_addr).await?;
        tracing::info!(?listener, "server listening");
        axum::serve(listener, router).await?;
        Ok(())
    }
}
