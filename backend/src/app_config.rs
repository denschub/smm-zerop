use std::net::SocketAddr;

use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use sqlx::postgres::PgConnectOptions;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub api_server: ApiServerConfig,
    pub database: DatabaseConfig,
    pub discord_bot_webhook: DiscordBotWebhookConfig,
    pub smm2_upstream_db: Smm2UpstreamDbConfig,
}

#[derive(Debug, Deserialize)]
pub struct ApiServerConfig {
    pub listen: SocketAddr,
    pub threads: Option<usize>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    #[serde_as(as = "DisplayFromStr")]
    pub connstring: PgConnectOptions,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct DiscordBotWebhookConfig {
    pub id: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct Smm2UpstreamDbConfig {
    pub connection_string: String,
}
