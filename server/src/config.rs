use std::{net::SocketAddr, time::Duration};

use anyhow::Result;
use atrium_api::types::string::Did;
use fred::prelude::ClientLike;
use url::Url;

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub atproto: AtpConfig,
    pub db: DbConfig,
    pub s3: S3Config,
    pub redis: RedisConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub bind_addr: SocketAddr,
}

#[derive(Debug, Clone)]
pub struct AtpConfig {
    pub did: Did,
    pub contact_email: String,
    pub relays: Vec<Url>,
    pub hostname: String,
}

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub name: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Clone)]
pub struct RedisConfig {
    client_cache: fred::prelude::Client,
    client_session: fred::prelude::Client,

    pub host: String,
    pub port: i32,
    pub db_cache: i16,
    pub db_session: i16,
}

impl RedisConfig {
    pub async fn new(host: String, port: i32, db_cache: i16, db_session: i16) -> Result<RedisConfig> {
        let cfg_cache = fred::prelude::Config::from_url(
            format!("redis://{}:{}/{}", host, port, db_cache).as_str()
        )?;
        let cfg_session = fred::prelude::Config::from_url(
            format!("redis://{}:{}/{}", host, port, db_session).as_str()
        )?;

        let client_cache = fred::prelude::Builder::from_config(cfg_cache)
            .with_connection_config(|config| {
                config.connection_timeout = Duration::from_secs(5);
            })
            .build()?;
        client_cache.init().await?;

        let client_session = fred::prelude::Builder::from_config(cfg_session)
            .with_connection_config(|config| {
                config.connection_timeout = Duration::from_secs(5);
            })
            .build()?;
        client_session.init().await?;

        Ok(RedisConfig {
            client_cache,
            client_session,
            host,
            port,
            db_cache,
            db_session,
        })
    }

    pub fn cache(&self) -> fred::prelude::Client {
        self.client_cache.clone()
    }

    pub fn session(&self) -> fred::prelude::Client {
        self.client_session.clone()
    }
}

#[derive(Debug, Clone)]
pub struct S3Config {
    pub bucket_name: String,
}
