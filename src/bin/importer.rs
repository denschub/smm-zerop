use anyhow::bail;
use clap::Parser;
use smm_zerop::{
    components::{settings::Settings, smm2_importer},
    get_db_pool, init_tracing,
};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use tracing::info;

fn main() -> anyhow::Result<()> {
    let settings = Settings::parse();
    if settings.upstream_db_connstring.is_none() {
        bail!("upstream_db_connstring must be configured for the importer to work!");
    }

    let mut rt = tokio::runtime::Builder::new_multi_thread();
    if let Some(threads) = settings.threads {
        rt.worker_threads(threads);
    }

    rt.enable_all()
        .build()?
        .block_on(async { run(settings).await })
}

async fn run(settings: Settings) -> anyhow::Result<()> {
    init_tracing(&settings);

    info!("connecting to upstream database...");
    let upstream_db_config =
        tiberius::Config::from_ado_string(&settings.upstream_db_connstring.unwrap())?;
    let upstream_db_tcp = TcpStream::connect(upstream_db_config.get_addr()).await?;
    upstream_db_tcp.set_nodelay(true)?;
    let mut upstream_db_client =
        tiberius::Client::connect(upstream_db_config, upstream_db_tcp.compat_write()).await?;

    info!("connecting to own database...");
    let own_db = get_db_pool(settings.database_url).await?;

    smm2_importer::run(&mut upstream_db_client, own_db).await?;
    Ok(())
}
