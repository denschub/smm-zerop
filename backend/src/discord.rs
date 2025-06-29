use std::collections::HashMap;

use anyhow::bail;
use tracing::error;

use crate::app_config::DiscordBotWebhookConfig;

fn lookup_message_data(
    config: &DiscordBotWebhookConfig,
    source: Option<&str>,
) -> (&'static str, String) {
    if let Some(source) = source {
        if let Some(appendix) = config.special_sources.get(source) {
            return ("api.smm-uncleared.com", format!(" ({appendix})"));
        }
    }

    ("smm-uncleared.com", String::new())
}

#[tracing::instrument(skip(config))]
pub async fn post_clear(
    config: &DiscordBotWebhookConfig,
    level_id: &str,
    source: Option<&str>,
) -> anyhow::Result<()> {
    let (username, appendix) = lookup_message_data(config, source);

    let mut body = HashMap::new();
    body.insert("username", username);

    let message = format!("!clear {level_id}{appendix}");
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
