use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

use crate::components::settings::{LogFormat, Settings};

pub mod components;
pub mod entities;
pub mod errors;
pub mod routers;

/// Creates a [PgPool] if possible. The pool has its max_connections value set
/// to mirror the tokio worker thread count.
pub async fn get_db_pool(connect_options: PgConnectOptions) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(
            tokio::runtime::Handle::current()
                .metrics()
                .num_workers()
                .try_into()
                .expect("num_workers to be less than 2^32"),
        )
        .connect_with(connect_options)
        .await
}

/// Sets up a [tracing_subscriber] according to the configurations provided vi
/// [Settings].
pub fn init_tracing(settings: &Settings) {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(settings.log_level.tracing_level())
        .with_target(false);
    match settings.log_format {
        LogFormat::Text => subscriber.with_ansi(false).init(),
        LogFormat::TextColor => subscriber.with_ansi(true).init(),
        LogFormat::Json => subscriber.json().with_span_list(false).init(),
    }
}
