use std::sync::Arc;

use chrono::{DateTime, Datelike, Utc};
use serde::Deserialize;
use tracing::{error, info};

use crate::{
    app_config::LevelImporterConfig,
    app_state::AppState,
    deserializers::{none_string_format, thecryptans_csv_datetime_format},
};

use super::level::Level;

#[derive(Debug, Deserialize)]
struct CsvRow {
    name: String,

    #[serde(with = "none_string_format")]
    description: Option<String>,

    id: String,

    attempts: i64,

    footprints: i64,

    #[serde(with = "none_string_format")]
    clear_condition: Option<String>,

    #[serde(with = "thecryptans_csv_datetime_format")]
    date: DateTime<Utc>,

    upload_time: i64,

    likes: i64,

    boos: i64,

    comments: i64,

    style: String,

    theme: String,

    #[serde(with = "none_string_format")]
    tag1: Option<String>,

    #[serde(with = "none_string_format")]
    tag2: Option<String>,
}

impl CsvRow {
    fn normalized_id(&self) -> String {
        self.id.trim().replace('-', "").to_lowercase()
    }

    fn normalized_tags(&self) -> Vec<String> {
        let mut tags = vec![self.tag1.clone(), self.tag2.clone()];
        tags.sort();
        tags.dedup();

        tags.iter()
            .filter_map(|o| o.clone())
            .map(|s| CsvRow::normalize_tag_name(&s))
            .collect()
    }

    fn normalize_tag_name(name: &str) -> String {
        name.trim().replace(' ', "_").to_lowercase()
    }
}

impl From<CsvRow> for Level {
    fn from(row: CsvRow) -> Self {
        Level {
            id: row.normalized_id(),
            year: row.date.year() as i64,

            title: row.name.clone(),
            description: row.description.clone(),
            uploaded_at: row.date,
            clearcheck_ms: row.upload_time,

            attempts: row.attempts,
            footprints: row.footprints,
            likes: row.likes,
            boos: row.boos,
            comments: row.comments,

            clear_condition: row.clear_condition.clone(),
            style: row.style.clone(),
            theme: CsvRow::normalize_tag_name(&row.theme),
            tags: row.normalized_tags(),
        }
    }
}

#[tracing::instrument(skip(app_state))]
pub async fn run(app_state: Arc<AppState>) -> anyhow::Result<()> {
    info!("loading levels...");
    let parsed_levels = load_levels(&app_state.config.level_importer).await?;
    let mut db_transaction = app_state.database.begin().await?;

    info!("deleting...");
    sqlx::query!("DELETE FROM levels_smm2")
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
    for source in &importer_config.csv_urls_smm2 {
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
