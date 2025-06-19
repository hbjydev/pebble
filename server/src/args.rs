use std::net::SocketAddr;

use atrium_api::types::string::Did;
use clap::Args;
use url::Url;

fn is_valid_did(s: &str) -> Result<Did, String> {
    match Did::new(s.to_string()) {
        Ok(did) => Ok(did),
        Err(_) => Err(format!("invalid did format: {}", s))
    }
}

#[derive(Args, Debug)]
pub struct PebbleServerArgs {
    #[arg(long, short, env = "PEBBLE_BIND_ADDR")]
    #[arg(default_value = "0.0.0.0:5990")]
    pub bind_addr: SocketAddr,

    #[arg(long, short, env = "PEBBLE_DID")]
    #[arg(value_parser = is_valid_did)]
    pub did: Did,

    #[arg(long, env = "PEBBLE_HOSTNAME")]
    pub hostname: String,

    #[arg(long, env = "PEBBLE_CONTACT_EMAIL")]
    pub contact_email: String,

    #[arg(long, default_value = "https://bsky.network", short, env = "PEBBLE_RELAYS")]
    pub relays: Vec<Url>,

    #[arg(long, env = "PEBBLE_DB_NAME")]
    pub db_name: String,
    #[arg(long, env = "PEBBLE_DB_USER")]
    pub db_user: String,
    #[arg(long, env = "PEBBLE_DB_PASSWORD")]
    pub db_password: String,
    #[arg(long, default_value = "localhost", env = "PEBBLE_DB_HOST")]
    pub db_host: String,
    #[arg(long, default_value_t = 5432, env = "PEBBLE_DB_PORT")]
    pub db_port: i32,

    #[arg(long, env = "PEBBLE_S3_BUCKET_NAME")]
    pub s3_bucket_name: String,
}
