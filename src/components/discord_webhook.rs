use std::collections::HashMap;

use anyhow::bail;
use tracing::error;

use crate::{
    components::app_state::AppState, entities::discord_webhook_source::DiscordWebhookSource,
};

async fn lookup_message_data(app_state: &AppState, source: Option<&str>) -> (&'static str, String) {
    if let Some(source) = source {
        match DiscordWebhookSource::get(&app_state.database, source).await {
            Ok(None) => {}
            Ok(Some(source)) => {
                return ("api.smm-uncleared.com", format!(" ({0})", source.url));
            }
            Err(err) => {
                error!("fetching discord webhook source failed: {:?}", err);
            }
        }
    }

    ("smm-uncleared.com", String::new())
}

#[tracing::instrument(skip(app_state))]
pub async fn post_clear(
    app_state: &AppState,
    level_id: &str,
    source: Option<&str>,
) -> anyhow::Result<()> {
    let (username, appendix) = lookup_message_data(app_state, source).await;

    let mut body = HashMap::new();
    body.insert("username", username);

    let message = format!("!clear {level_id}{appendix}");
    body.insert("content", &message);

    let discord_response = reqwest::Client::new()
        .post(format!(
            "https://discord.com/api/webhooks/{}/{}",
            app_state.settings.discord_webhook_id, app_state.settings.discord_webhook_token
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
