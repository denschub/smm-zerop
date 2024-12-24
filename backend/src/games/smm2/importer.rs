use std::collections::HashSet;

use chrono::{Datelike, Utc};
use futures_util::TryStreamExt;
use sqlx::PgPool;
use tracing::info;

use super::level::Level;

macro_rules! expect_not_null {
    ($value:expr, $field:expr) => {
        $value
            .get($field)
            .expect(&format!("{} IS NOT NULL", $field))
    };
    ($value:expr, $field:expr, $type:ty) => {
        $value
            .get::<$type, &str>($field)
            .expect(&format!("{} IS NOT NULL", $field))
    };
}

fn normalized_id(id: &str) -> String {
    id.trim().replace('-', "").to_lowercase()
}

fn normalized_tags(tag1: Option<&str>, tag2: Option<&str>) -> Vec<String> {
    let mut tags = vec![tag1, tag2];
    tags.sort();
    tags.dedup();

    tags.iter()
        .filter_map(|o| *o)
        .filter(|t| *t != "None")
        .map(normalize_tag_name)
        .collect()
}

fn normalize_tag_name(name: &str) -> String {
    name.trim().replace(' ', "_").to_lowercase()
}

impl From<tiberius::Row> for Level {
    fn from(value: tiberius::Row) -> Self {
        let id: &str = expect_not_null!(value, "id");
        let uploaded_at: chrono::DateTime<Utc> = expect_not_null!(value, "date");

        let title: &str = expect_not_null!(value, "name");
        let description = value.get::<&str, &str>("description").map(|s| s.to_owned());

        let clearcheck_ms: i32 = expect_not_null!(value, "upload_time");
        let attempts: i32 = expect_not_null!(value, "attempts");
        let footprints: i32 = expect_not_null!(value, "footprints");

        // Please do not ask me why those fields are strings...
        let likes: i64 = expect_not_null!(value, "likes", &str)
            .parse::<i64>()
            .expect("field should be a numeric value");
        let boos: i64 = expect_not_null!(value, "boos", &str)
            .parse::<i64>()
            .expect("field should be a numeric value");
        let comments: i64 = expect_not_null!(value, "comments", &str)
            .parse::<i64>()
            .expect("field should be a numeric value");

        let clear_condition = match value.get::<i32, &str>("clear_condition") {
            None => None,
            Some(0) => None,
            Some(i) => Some(i as i64),
        };
        let clear_condition_magnitude =
            match expect_not_null!(value, "clear_condition_magnitude", i32) {
                0 => None,
                i => Some(i as i64),
            };

        let style: &str = expect_not_null!(value, "style");
        let theme: &str = expect_not_null!(value, "theme");
        let tags = normalized_tags(value.get("tag1"), value.get("tag2"));

        Level {
            id: normalized_id(id),
            year: uploaded_at.year() as i64,
            title: title.to_owned(),
            description,
            uploaded_at,
            clearcheck_ms: clearcheck_ms as i64,
            attempts: attempts as i64,
            footprints: footprints as i64,
            likes,
            boos,
            comments,
            clear_condition,
            clear_condition_magnitude,
            style: style.to_owned(),
            theme: normalize_tag_name(theme),
            tags,
        }
    }
}

#[tracing::instrument(skip(upstream_db, own_db))]
pub async fn run(
    upstream_db: &mut tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
    own_db: PgPool,
) -> anyhow::Result<()> {
    let level_blocklist: HashSet<String> =
        sqlx::query_scalar!("SELECT level_id FROM level_blocklist WHERE game = 'smm2'")
            .fetch_all(&own_db)
            .await?
            .iter()
            .cloned()
            .collect();

    let mut db_transaction = own_db.begin().await?;

    info!("deleting current levels...");
    sqlx::query!("DELETE FROM levels_smm2")
        .execute(&mut *db_transaction)
        .await?;

    info!("streaming levels...");
    let mut levels_count = 0;
    let mut rows = upstream_db
        .simple_query("SELECT * FROM v_Uncleared")
        .await?
        .into_row_stream();
    while let Some(row) = rows.try_next().await? {
        let level: Level = row.into();

        if level_blocklist.contains(&level.id) {
            continue;
        }

        level.store(&mut *db_transaction).await?;
        levels_count += 1;
    }

    if levels_count > 0 {
        info!("committing...");
        db_transaction.commit().await?;
        info!("done, imported {} levels!", levels_count);
    } else {
        info!("somehow got no levels, rolling back...");
        db_transaction.rollback().await?;
        info!("done!");
    }

    Ok(())
}
