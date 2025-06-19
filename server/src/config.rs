use std::net::SocketAddr;

use atrium_api::types::string::Did;
use url::Url;

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub atproto: AtpConfig,
    pub db: DbConfig,
    pub s3: S3Config,
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
pub struct S3Config {
    pub bucket_name: String,
}
