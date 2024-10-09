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

    pub clear_condition: Option<String>,
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
                style,
                theme,
                tags
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
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
                style,
                theme,
                tags
            FROM levels_smm2
            WHERE 1 = 1",
        );

        query.push(" AND year = ");
        query.push_bind(params.year);

        push_optional_filter!(query, params.min_attempts, " AND attempts >= ");
        push_optional_filter!(query, params.max_attempts, " AND attempts <= ");
        push_optional_filter!(query, params.min_footprints, " AND footprints >= ");
        push_optional_filter!(query, params.max_footprints, " AND footprints <= ");
        push_optional_filter!(query, params.min_clearcheck_ms, " AND clearcheck_ms >= ");
        push_optional_filter!(query, params.max_clearcheck_ms, " AND clearcheck_ms <= ");
        push_optional_filter!(query, &params.style, " AND style =  ");
        push_optional_filter!(query, &params.theme, " AND theme =  ");
        push_optional_filter!(query, &params.tag, " AND ", " = ANY(tags)");

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
pub struct FilterParams {
    pub year: i64,
    pub min_attempts: Option<i64>,
    pub max_attempts: Option<i64>,
    pub min_footprints: Option<i64>,
    pub max_footprints: Option<i64>,
    pub min_clearcheck_ms: Option<i64>,
    pub max_clearcheck_ms: Option<i64>,
    pub style: Option<Style>,
    pub theme: Option<Theme>,
    pub tag: Option<Tag>,
}
