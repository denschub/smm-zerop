use std::{sync::Arc, time::Duration};

use anyhow::{bail, Context};
use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use smm_zerop_backend::{
    app_config::AppConfig,
    app_state::AppState,
    games::{smm1, smm2},
};
use sqlx::postgres::PgPoolOptions;
use tokio::time::sleep;
use tower_http::cors::{self, CorsLayer};
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Usage: smm-zerop-backend ./path/to/config.toml");
    }

    let config_file_contents =
        std::fs::read_to_string(&args[1]).context("Failed to load config file")?;
    let app_config: AppConfig =
        toml::from_str(&config_file_contents).context("Failed to parse config file")?;
    let app_config_arc = Arc::new(app_config);

    let db_pool = PgPoolOptions::new()
        .max_connections(app_config_arc.database.max_connections)
        .connect_with(app_config_arc.database.connstring.clone())
        .await
        .context("Failed to connect to database")?;

    let app_state = AppState {
        config: app_config_arc.clone(),
        database: db_pool,
    };
    let app_state_arc = Arc::new(app_state);

    tokio::select! {
        _ = shutdown_signal() => {},
        _ = run_http_server(app_state_arc.clone()) => {},
        _ = level_import_loop(app_state_arc.clone()), if app_config_arc.clone().level_importer.run_inline => {},
    }

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

#[tracing::instrument(skip(app_state))]
async fn level_import_loop(app_state: Arc<AppState>) {
    loop {
        let start_of_loop = chrono::Utc::now();
        if let Err(err) = smm1::importer::run(app_state.clone()).await {
            error!("smm1 import failed: {:?}", err);
        }

        if let Err(err) = smm2::importer::run(app_state.clone()).await {
            error!("smm2 import failed: {:?}", err);
        }

        let end_of_loop = chrono::Utc::now();
        let loop_runtime = end_of_loop - start_of_loop;
        info!("import duration: {}", loop_runtime);
        sleep(
            Duration::from_secs(app_state.config.level_importer.interval)
                - loop_runtime
                    .to_std()
                    .expect("loop runtime cannot be negative"),
        )
        .await;
    }
}

async fn run_http_server(app_state: Arc<AppState>) -> anyhow::Result<()> {
    let cors_layer = CorsLayer::new()
        .allow_methods(cors::Any)
        .allow_origin(cors::Any);

    let app = Router::new()
        .route("/__heartbeat__", get(get_heartbeat))
        .route("/smm1/random_level", get(get_smm1_random_level))
        .route("/smm2/random_level", get(get_smm2_random_level))
        .layer(cors_layer)
        .with_state(app_state.clone());

    axum::Server::bind(&app_state.config.api_server.listen)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[tracing::instrument()]
async fn get_heartbeat() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

#[debug_handler]
#[tracing::instrument(skip(app_state))]
async fn get_smm1_random_level(
    params: Query<smm1::level::FilterParams>,
    State(app_state): State<Arc<AppState>>,
) -> Response {
    let random_level_result =
        smm1::level::Level::get_random_level(&app_state.database, &params).await;

    if let Err(err) = random_level_result {
        error!("{:?}", err);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if let Ok(Some(result)) = random_level_result {
        return Json(result).into_response();
    }

    StatusCode::NOT_FOUND.into_response()
}

#[debug_handler]
#[tracing::instrument(skip(app_state))]
async fn get_smm2_random_level(
    params: Query<smm2::level::FilterParams>,
    State(app_state): State<Arc<AppState>>,
) -> Response {
    let random_level_result =
        smm2::level::Level::get_random_level(&app_state.database, &params).await;

    if let Err(err) = random_level_result {
        error!("{:?}", err);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if let Ok(Some(result)) = random_level_result {
        return Json(result).into_response();
    }

    StatusCode::NOT_FOUND.into_response()
}
