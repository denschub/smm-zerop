use std::sync::Arc;

use chrono::{Datelike, NaiveDate};
use serde::Deserialize;
use tracing::{error, info};

use crate::{
    app_config::LevelImporterConfig,
    app_state::AppState,
    deserializers::{gsheets_csv_date_format, thousands_seperated_integer},
};

use super::level::Level;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CsvRow {
    title: String,

    #[serde(rename = "Upload Date", with = "gsheets_csv_date_format")]
    upload_date: NaiveDate,

    #[serde(with = "thousands_seperated_integer")]
    stars: i64,

    #[serde(with = "thousands_seperated_integer")]
    players: i64,

    #[serde(with = "thousands_seperated_integer")]
    attempts: i64,

    #[serde(rename = "Level ID")]
    level_id: String,
}

impl CsvRow {
    fn normalized_id(&self) -> String {
        self.level_id.trim().replace('-', "").to_lowercase()
    }
}

impl From<CsvRow> for Level {
    fn from(row: CsvRow) -> Self {
        Level {
            id: row.normalized_id(),
            year: row.upload_date.year() as i64,

            title: row.title,
            uploaded_at: row.upload_date,

            attempts: row.attempts,
            footprints: row.players,
            likes: row.stars,
        }
    }
}

#[tracing::instrument(skip(app_state))]
pub async fn run(app_state: Arc<AppState>) -> anyhow::Result<()> {
    info!("loading levels...");
    let parsed_levels = load_levels(&app_state.config.level_importer).await?;
    let mut db_transaction = app_state.database.begin().await?;

    info!("deleting...");
    sqlx::query!("DELETE FROM levels_smm1")
        .execute(&mut *db_transaction)
        .await?;

    info!("inserting...");
    for level in parsed_levels {
        level.store(&mut *db_transaction).await?;
    }

    info!("committing...");
    db_transaction.commit().await?;

    info!("done!");
    Ok(())
}

async fn load_levels(importer_config: &LevelImporterConfig) -> anyhow::Result<Vec<Level>> {
    let mut levels: Vec<Level> = vec![];
    for source in &importer_config.csv_urls_smm1 {
        let mut fetched = parse_csv(source, importer_config).await?;
        levels.append(&mut fetched);
    }

    Ok(levels)
}

#[tracing::instrument()]
async fn parse_csv(url: &str, importer_config: &LevelImporterConfig) -> anyhow::Result<Vec<Level>> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(importer_config.accept_invalid_ssl)
        .user_agent(&importer_config.user_agent)
        .build()
        .expect("passed parameters are known to be set");

    let csv_data = client.get(url).send().await?.text().await?;
    let mut csv_reader = csv::Reader::from_reader(csv_data.as_bytes());

    Ok(csv_reader
        .deserialize::<CsvRow>()
        .filter_map(|r| match r {
            Ok(row) => Some(row),
            Err(err) => {
                error!("{:?}", err);
                None
            }
        })
        .map(|r| r.into())
        .collect())
}
