use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use tokio::net::TcpListener;
use tracing::info;

use smm_zerop::{
    components::{app_state::AppState, lazyjinja::LazyJinja, settings::Settings},
    get_db_pool, init_tracing,
    routers::build_main_router,
};

/// Sets up a relevant shutdown signals. This will exit on either SIGINT
/// (aka Ctrl+C) or SIGTERM.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to create Ctrl+C handler");
    };

    let sigterm = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to create SIGTERM handler")
            .recv()
            .await;
    };

    tokio::select! {
        () = ctrl_c => {},
        () = sigterm => {},
    }

    info!("shutdown signal received")
}

fn main() -> anyhow::Result<()> {
    let settings = Settings::parse();

    let mut rt = tokio::runtime::Builder::new_multi_thread();
    if let Some(threads) = settings.threads {
        rt.worker_threads(threads);
    }

    rt.enable_all()
        .build()?
        .block_on(async { run(settings).await })
}

async fn run(settings: Settings) -> anyhow::Result<()> {
    let settings_clone = settings.clone();
    init_tracing(&settings);

    let database = get_db_pool(settings_clone.database_url).await?;
    sqlx::migrate!().run(&database).await?;

    let router = build_main_router(AppState {
        database,
        settings: Arc::new(settings),
        template: Arc::new(LazyJinja::new()),
    });

    let listener = TcpListener::bind(settings_clone.listen)
        .await
        .context(format!("could not listen to `{}`", settings_clone.listen))?;

    info!("starting server on `{}`", settings_clone.listen);
    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("failed to start server")?;

    Ok(())
}
