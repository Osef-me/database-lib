//! Reusable macros to generate common SQLx queries without boilerplate.
//!
//! These macros leverage `sqlx::QueryBuilder` so we don't need fully literal SQL
//! at compile-time (which `query!`/`query_as!` require).

/// Define an async function that fetches a row by its `id` using `sqlx::QueryBuilder`.
///
/// Parameters:
/// - `fn_name`: function name to generate
/// - `table_lit`: table name as a string literal, e.g. "score_metadata"
/// - `RowType`: fully-qualified row type path
/// - `columns_lit`: columns to select as a string literal
///
/// Example:
/// define_by_id!(find_by_id, "score_metadata", crate::models::score_metadata::types::ScoreMetadataRow,
///     "id, skin, pause_count, started_at, ended_at, time_paused, score, accuracy, max_combo, perfect, count_300, count_100, count_50, count_miss, count_katu, count_geki, created_at");
#[macro_export]
macro_rules! define_by_id {
    ( $fn_name:ident, $table_lit:literal, $RowType:path, $columns_lit:literal ) => {
        pub async fn $fn_name(
            pool: &sqlx::PgPool,
            id: i32,
        ) -> Result<Option<$RowType>, sqlx::Error> {
            let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT ");
            builder.push($columns_lit);
            builder.push(" FROM ");
            builder.push($table_lit);
            builder.push(" WHERE id = ");
            builder.push_bind(id);
            builder
                .build_query_as::<$RowType>()
                .fetch_optional(pool)
                .await
        }
    };
}

/// Define an async INSERT function that returns the auto-generated `id` (i32).
///
/// Parameters:
/// - `fn_name`: function name to generate
/// - `table_lit`: table name as a string literal
/// - `RowType`: fully-qualified row type path of the input struct
/// - field list: one or more identifiers which are taken from `row.field` in order
///
/// Example:
/// define_insert_returning_id!(insert, "score_metadata", crate::models::score_metadata::types::ScoreMetadataRow,
///     skin, pause_count, started_at, ended_at, time_paused, score, accuracy, max_combo, perfect, count_300, count_100, count_50, count_miss, count_katu, count_geki);
#[macro_export]
macro_rules! define_insert_returning_id {
    ( $fn_name:ident, $table_lit:literal, $RowType:path, $($field:ident),+ $(,)? ) => {
        pub async fn $fn_name(
            pool: &sqlx::PgPool,
            row: $RowType,
        ) -> Result<i32, sqlx::Error> {
            let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("INSERT INTO ");
            builder.push($table_lit);
            builder.push(" (");
            builder.push( stringify!($($field),+) );
            builder.push(", created_at) VALUES (");
            let mut separated = builder.separated(", ");
            $( separated.push_bind(row.$field); )+
            separated.push("NOW()");
            // let `separated` drop naturally; explicit drop is unnecessary
            builder.push(") RETURNING id");

            let rec = builder.build().fetch_one(pool).await?;
            use sqlx::Row;
            let id: i32 = rec.try_get("id")?;
            Ok(id)
        }
    };
}

// Internal helper to generate a placeholder list like "$1, $2, $3"
// Note: sqlx::query! requires a fully literal SQL string. Building a dynamic
// placeholder list within concat! isn't straightforward in macro_rules without
// counting. Therefore, for portability across many Rust versions and to keep
// things simple, prefer the `define_insert_returning_row!` variant where the
// caller provides the RETURNING columns list. For returning id, we will emit a
// concrete, explicit SQL through another macro.

/// Define an async INSERT function that returns the full row using `QueryBuilder`.
/// The caller provides the explicit RETURNING columns list literal.
///
/// Parameters:
/// - `fn_name`: function name to generate
/// - `table_lit`: table name as a string literal
/// - `RowType`: fully-qualified row type path to be returned
/// - field list: one or more identifiers which are taken from `row.field` in order
/// - `;` then `returning_columns_lit`: string literal of columns to return
///
/// Example:
/// define_insert_returning_row!(insert, "score", crate::models::score::types::ScoreRow,
///     user_id, beatmap_id, score_metadata_id, replay_id, rate, hwid, mods, hash, rank, status;
///     "id, user_id, beatmap_id, score_metadata_id, replay_id, rate, hwid, mods, hash, rank, status, created_at");
#[macro_export]
macro_rules! define_insert_returning_row {
    ( $fn_name:ident, $table_lit:literal, $RowType:path, $($field:ident),+ ; $returning_columns_lit:literal ) => {
        pub async fn $fn_name(
            pool: &sqlx::PgPool,
            row: $RowType,
        ) -> Result<$RowType, sqlx::Error> {
            let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("INSERT INTO ");
            builder.push($table_lit);
            builder.push(" (");
            builder.push( stringify!($($field),+) );
            builder.push(", created_at) VALUES (");
            let mut separated = builder.separated(", ");
            $( separated.push_bind(row.$field); )+
            separated.push("NOW()");
            // let `separated` drop naturally; explicit drop is unnecessary
            builder.push(") RETURNING ");
            builder.push($returning_columns_lit);

            builder
                .build_query_as::<$RowType>()
                .fetch_one(pool)
                .await
        }
    };
}
