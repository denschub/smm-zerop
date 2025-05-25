use std::vec;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, FromRow, PgExecutor, Postgres, QueryBuilder};

use crate::push_optional_filter;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Level {
    pub id: String,
    pub year: i64,

    pub title: String,
    pub description: Option<String>,
    pub uploaded_at: DateTime<Utc>,
    pub clearcheck_ms: i64,

    pub attempts: i64,
    pub footprints: i64,
    pub likes: i64,
    pub boos: i64,
    pub comments: i64,

    pub clear_condition: Option<i64>,
    pub clear_condition_magnitude: Option<i64>,

    pub style: String,
    pub theme: String,
    pub tags: Vec<String>,
}

impl Level {
    pub async fn store<'a, Executor: PgExecutor<'a>>(
        &self,
        executor: Executor,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO levels_smm2 (
                id,
                year,
                title,
                description,
                uploaded_at,
                clearcheck_ms,
                attempts,
                footprints,
                likes,
                boos,
                comments,
                clear_condition,
                clear_condition_magnitude,
                style,
                theme,
                tags
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT DO NOTHING",
            self.id,
            self.year,
            self.title,
            self.description,
            self.uploaded_at,
            self.clearcheck_ms,
            self.attempts,
            self.footprints,
            self.likes,
            self.boos,
            self.comments,
            self.clear_condition,
            self.clear_condition_magnitude,
            self.style,
            self.theme,
            &self.tags,
        )
        .execute(executor)
        .await
    }

    pub async fn get_random_level<'a, Executor: PgExecutor<'a>>(
        executor: Executor,
        params: &FilterParams,
    ) -> Result<Option<Level>, sqlx::Error> {
        let mut query = QueryBuilder::<Postgres>::new(
            "SELECT
                id,
                year,
                title,
                description,
                uploaded_at,
                clearcheck_ms,
                attempts,
                footprints,
                likes,
                boos,
                comments,
                clear_condition,
                clear_condition_magnitude,
                style,
                theme,
                tags
            FROM levels_smm2
            WHERE 1 = 1",
        );

        push_optional_filter!(query, params.year, " AND year = ");
        push_optional_filter!(query, params.min_attempts, " AND attempts >= ");
        push_optional_filter!(query, params.max_attempts, " AND attempts <= ");
        push_optional_filter!(query, params.min_footprints, " AND footprints >= ");
        push_optional_filter!(query, params.max_footprints, " AND footprints <= ");
        push_optional_filter!(query, params.min_clearcheck_ms, " AND clearcheck_ms >= ");
        push_optional_filter!(query, params.max_clearcheck_ms, " AND clearcheck_ms <= ");
        push_optional_filter!(query, &params.style, " AND style =  ");
        push_optional_filter!(query, &params.theme, " AND theme =  ");
        push_optional_filter!(query, &params.tag, " AND ", " = ANY(tags)");

        if let Some(cc_filter) = &params.clear_condition_group {
            match cc_filter.id_list() {
                None => {
                    query.push(" AND clear_condition IS NULL");
                }
                Some(ids) => {
                    query.push(format!(
                        " AND clear_condition IN({})",
                        ids.iter()
                            .map(|i| i.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    ));
                }
            }
        }

        query.push(" ORDER BY random() LIMIT 1");

        query
            .build_query_as::<Level>()
            .fetch_optional(executor)
            .await
    }

    pub async fn id_exists<'a, Executor: PgExecutor<'a>>(
        executor: Executor,
        level_id: &str,
    ) -> bool {
        sqlx::query!("SELECT id FROM levels_smm2 WHERE id = $1 LIMIT 1", level_id)
            .fetch_optional(executor)
            .await
            .is_ok_and(|r| r.is_some())
    }

    pub fn normalized_internal_level_id(raw_id: &str) -> String {
        raw_id.trim().replace('-', "").to_lowercase()
    }

    pub fn formatted_level_id(raw_id: &str) -> String {
        raw_id
            .to_uppercase()
            .chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("-")
    }
}

#[derive(Debug, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "text", rename_all = "UPPERCASE")]
pub enum Style {
    SMB1,
    SMB3,
    SMW,
    NSMBU,
    SM3DW,
}

#[derive(Debug, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum Theme {
    Airship,
    Castle,
    Desert,
    Forest,
    GhostHouse,
    Overworld,
    Sky,
    Snow,
    Underground,
}

#[derive(Debug, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum Tag {
    Art,
    AutoMario,
    Autoscroll,
    BossBattle,
    Link,
    MultiplayerVersus,
    Music,
    PuzzleSolving,
    Shooter,
    ShortAndSweet,
    SinglePlayer,
    Speedrun,
    Standard,
    Technical,
    Themed,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClearConditionGroup {
    None,
    NoJumping,
    NoDamage,
    DefeatingEnemies,
    PowerupFinish,
    HoldingActivating,
    Collecting,
}

#[derive(Debug, Deserialize)]
pub struct FilterParams {
    pub year: Option<i64>,
    pub min_attempts: Option<i64>,
    pub max_attempts: Option<i64>,
    pub min_footprints: Option<i64>,
    pub max_footprints: Option<i64>,
    pub min_clearcheck_ms: Option<i64>,
    pub max_clearcheck_ms: Option<i64>,
    pub clear_condition_group: Option<ClearConditionGroup>,
    pub style: Option<Style>,
    pub theme: Option<Theme>,
    pub tag: Option<Tag>,
}

impl ClearConditionGroup {
    fn id_list(&self) -> Option<Vec<i64>> {
        match self {
            Self::None => None,
            Self::NoJumping => Some(vec![1]),
            Self::NoDamage => Some(vec![4]),
            Self::DefeatingEnemies => Some(vec![
                2, 3, 9, 11, 14, 15, 17, 18, 19, 20, 22, 24, 25, 26, 27, 29, 30, 32, 33, 34, 36,
                39, 40, 42, 43, 44, 45, 46, 50, 51, 52, 54, 55, 61, 65, 67, 68, 69, 70, 71, 72, 77,
                78, 79, 80, 81, 83, 85, 87, 92, 93,
            ]),
            Self::PowerupFinish => Some(vec![
                5, 6, 7, 8, 10, 12, 13, 21, 28, 41, 47, 48, 49, 53, 57, 58, 59, 60, 62, 63, 64, 73,
                74, 75, 76, 84, 89, 90, 91,
            ]),
            Self::HoldingActivating => Some(vec![16, 23, 31, 37, 38]),
            Self::Collecting => Some(vec![35, 56, 66, 82, 86, 88]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatted_level_id_formats_correctly() {
        assert_eq!(Level::formatted_level_id("abc123DEF"), "ABC-123-DEF")
    }
}
