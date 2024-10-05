use anyhow::{bail, Context};
use smm_zerop_backend::{app_config::AppConfig, games::smm2::importer};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Usage: smm2-importer ./path/to/config.toml");
    }

    let config_file_contents =
        std::fs::read_to_string(&args[1]).context("Failed to load config file")?;
    let app_config: AppConfig =
        toml::from_str(&config_file_contents).context("Failed to parse config file")?;

    info!("connecting to upstream database...");
    let upstream_db_config =
        tiberius::Config::from_ado_string(&app_config.smm2_upstream_db.connection_string)?;
    let upstream_db_tcp = TcpStream::connect(upstream_db_config.get_addr()).await?;
    upstream_db_tcp.set_nodelay(true)?;
    let mut upstream_db_client =
        tiberius::Client::connect(upstream_db_config, upstream_db_tcp.compat_write()).await?;

    info!("connecting to own database...");
    let db_pool = PgPoolOptions::new()
        .max_connections(app_config.database.max_connections)
        .connect_with(app_config.database.connstring.clone())
        .await
        .context("Failed to connect to database")?;

    importer::run(&mut upstream_db_client, db_pool).await?;

    Ok(())
}
