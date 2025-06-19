use std::net::SocketAddr;

use atrium_api::types::string::Did;
use url::Url;

pub struct Config {
    pub server: ServerConfig,
    pub atproto: AtpConfig,
    pub db: DbConfig,
    pub s3: S3Config,
}

pub struct ServerConfig {
    pub bind_addr: SocketAddr,
}

pub struct AtpConfig {
    pub did: Did,
    pub contact_email: String,
    pub relays: Vec<Url>,
}

pub struct DbConfig {
    pub name: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: i32,
}

pub struct S3Config {
    pub bucket_name: String,
}
