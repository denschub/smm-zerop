use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, FromRow, PgExecutor, Postgres, QueryBuilder};

use crate::push_optional_filter;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Level {
    pub id: String,
    pub year: i64,

    pub title: String,
    pub uploaded_at: NaiveDate,

    pub attempts: i64,
    pub footprints: i64,
    pub likes: i64,
}

impl Level {
    pub async fn store<'a, Executor: PgExecutor<'a>>(
        &self,
        executor: Executor,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO levels_smm1 (
                id,
                year,
                title,
                uploaded_at,
                attempts,
                footprints,
                likes
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT DO NOTHING",
            self.id,
            self.year,
            self.title,
            self.uploaded_at,
            self.attempts,
            self.footprints,
            self.likes,
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
                uploaded_at,
                attempts,
                footprints,
                likes
            FROM levels_smm1
            WHERE 1 = 1",
        );

        query.push(
            " AND NOT EXISTS (
                SELECT NULL FROM known_cleared
                WHERE game = 'smm1' AND id = levels_smm1.id
            )",
        );

        query.push(
            " AND NOT EXISTS (
                SELECT NULL FROM level_reservations
                WHERE game = 'smm1' AND level_id = levels_smm1.id
            )",
        );

        query.push(" AND year = ");
        query.push_bind(params.year);

        push_optional_filter!(query, params.min_attempts, " AND attempts >= ");
        push_optional_filter!(query, params.max_attempts, " AND attempts <= ");

        query.push(" ORDER BY random() LIMIT 1");

        query
            .build_query_as::<Level>()
            .fetch_optional(executor)
            .await
    }
}

#[derive(Debug, Deserialize)]
pub struct FilterParams {
    pub year: i64,
    pub min_attempts: Option<i64>,
    pub max_attempts: Option<i64>,
}
