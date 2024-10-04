use std::net::SocketAddr;

use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use sqlx::postgres::PgConnectOptions;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub api_server: ApiServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct ApiServerConfig {
    pub listen: SocketAddr,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    #[serde_as(as = "DisplayFromStr")]
    pub connstring: PgConnectOptions,
    pub max_connections: u32,
}
