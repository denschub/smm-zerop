use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgExecutor};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DiscordWebhookSource {
    pub key: String,
    pub url: String,
}

impl DiscordWebhookSource {
    pub async fn get<'a, Executor: PgExecutor<'a>>(
        executor: Executor,
        key: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "SELECT key, url FROM discord_webhook_sources WHERE key = $1",
            key
        )
        .fetch_optional(executor)
        .await
    }
}
