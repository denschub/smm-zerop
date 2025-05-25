use std::collections::HashMap;

use anyhow::bail;
use tracing::error;

use crate::app_config::DiscordBotWebhookConfig;

#[tracing::instrument(skip(config))]
pub async fn post_clear(config: &DiscordBotWebhookConfig, level_id: &str) -> anyhow::Result<()> {
    let mut body = HashMap::new();
    body.insert("username", "smm-uncleared.com");

    let message = format!("!clear {}", level_id);
    body.insert("content", &message);

    let discord_response = reqwest::Client::new()
        .post(format!(
            "https://discord.com/api/webhooks/{}/{}",
            config.id, config.token
        ))
        .json(&body)
        .send()
        .await?;

    if !discord_response.status().is_success() {
        error!("got status `{}`", discord_response.status());
        bail!("discord webhook failed: unexpected response status");
    }

    Ok(())
}
