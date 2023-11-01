use std::sync::Arc;

use sqlx::PgPool;

use crate::app_config::AppConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub database: PgPool,
}
