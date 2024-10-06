use std::sync::Arc;

use anyhow::{bail, Context};
use axum::{
    debug_handler,
    extract::{self, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use smm_zerop_backend::{app_config::AppConfig, app_state::AppState, discord, games::smm2};
use sqlx::postgres::PgPoolOptions;
use tokio::{net::TcpListener, signal};
use tower_http::cors::{self, CorsLayer};
use tracing::error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Usage: api-server ./path/to/config.toml");
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

    run_http_server(app_state_arc.clone()).await
}

async fn shutdown_signal() {
    let sigint = async {
        signal::unix::signal(signal::unix::SignalKind::interrupt())
            .expect("creating SIGINT handler should not fail")
            .recv()
            .await;
    };

    let sigterm = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("creating SIGTERM handler should not fail")
            .recv()
            .await;
    };

    tokio::select! {
        () = sigint => {},
        () = sigterm => {},
    }
}

async fn run_http_server(app_state: Arc<AppState>) -> anyhow::Result<()> {
    let cors_layer = CorsLayer::new()
        .allow_methods(cors::Any)
        .allow_origin(cors::Any);

    let app = Router::new()
        .route("/__heartbeat__", get(get_heartbeat))
        .route("/smm2/random_level", get(get_smm2_random_level))
        .route("/smm2/mark_cleared", post(post_smm2_mark_cleared))
        .layer(cors_layer)
        .with_state(app_state.clone());

    let listener = TcpListener::bind(&app_state.config.api_server.listen).await?;
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

#[tracing::instrument()]
async fn get_heartbeat() -> impl IntoResponse {
    StatusCode::NO_CONTENT
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

#[derive(Debug, Deserialize)]
struct PostSmm2MarkClearedPayload {
    level_id: String,
}

#[debug_handler]
#[tracing::instrument(skip(app_state))]
async fn post_smm2_mark_cleared(
    State(app_state): State<Arc<AppState>>,
    extract::Json(payload): extract::Json<PostSmm2MarkClearedPayload>,
) -> Response {
    if payload.level_id.len() != 9 {
        return (StatusCode::BAD_REQUEST, "invalid level id").into_response();
    }

    match discord::post_clear(&app_state.config.discord_bot_webhook, &payload.level_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()).into_response(),
    }
}
