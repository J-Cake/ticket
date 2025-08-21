use std::net::SocketAddr;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use clap::Parser;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub auth: Authentication,

    #[serde(default)]
    pub hooks: Vec<Hook>
}

#[derive(Serialize, Deserialize)]
pub struct Server {
    pub listen: SocketAddr,
}

#[derive(Serialize, Deserialize)]
pub struct Authentication {
    pub jwks_url: String,

    pub max_certificate_age: u64,

    pub audience: String,
    pub issuer: String,

    pub token_ttl: u64,
}


#[derive(Serialize, Deserialize)]
pub struct Hook {
    pub event: String,
}

#[derive(Parser)]
pub struct CliArgs {
    #[arg(long = "config", short = 'c', default_value = "docker.toml")]
    pub config: PathBuf
}