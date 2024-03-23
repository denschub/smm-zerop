use std::net::SocketAddr;

use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use sqlx::postgres::PgConnectOptions;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub api_server: ApiServerConfig,
    pub database: DatabaseConfig,
    pub level_importer: LevelImporterConfig,
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

#[derive(Debug, Deserialize)]
pub struct LevelImporterConfig {
    pub run_inline: bool,
    pub interval: u64,
    pub user_agent: String,
    pub accept_invalid_ssl: bool,
    pub csv_urls_smm2: Vec<String>,
}
