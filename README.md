## database-lib (Rust, sqlx, Postgres)

### Summary

- [Overview](#overview)
- [Project structure](#project-structure)
- [Environment configuration (.env)](#environment-configuration-env)
- [Install and build](#install-and-build)
- [How to use (quick examples)](#how-to-use-quick-examples)
- [Other available domains (similar API)](#other-available-domains-similar-api)
- [Tracing and logging](#tracing-and-logging)
- [Notes](#notes)

## Overview

This crate provides typed data-access helpers around sqlx for Osef.me Databases. It exposes Rust model modules (beatmap, score, replay, etc.) with:
- type-safe row structs (serde-compatible)
- validators
- query functions (find/insert/update/exists)
- small macros to reduce SQL boilerplate

The crate does not create database connections itself. You pass a `sqlx::PgPool` from your application.

## Project structure

- **`src/lib.rs`**: exports `macros`, `models`, `utils`
- **`src/macros.rs`**: reusable macros to generate common query functions with `sqlx::QueryBuilder`
- **`src/utils.rs`**: reusable regex validators
- **`src/models/`**: one folder per table/domain. Each has:
  - **`types.rs`**: row types (`#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]`-friendly)
  - **`validators.rs`**: field validation when relevant
  - **`impl.rs`**: ergonomic methods that call query functions
  - **`query/`**: concrete SQLx queries (by_id, insert, exists, search, ...)
  - **`tests/`**: basic validation and model tests

## Environment configuration (.env)

While this library does not load .env by itself, most applications provide a Postgres connection URL via `DATABASE_URL`.

Example `.env`:

```bash
DATABASE_URL=postgres://USER:PASSWORD@HOST:5432/DB_NAME
# Optional for logging, used by tracing-subscriber in consumers
RUST_LOG=info
```

Add dotenv loading in your executable if desired:

```rust
// in your binary crate (not this library)
dotenvy::dotenv().ok();
let database_url = std::env::var("DATABASE_URL")?;
let pool = sqlx::postgres::PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await?;
```

## Install and build

- Rust: stable toolchain (2021 edition)
- Database: Postgres 13+

```bash
cargo build
cargo test
```

## How to use (quick examples)

Bring the crate into your application and pass a `PgPool` to the model methods.

```rust
use db::models::score::types::ScoreRow;
use db::models::score::ScoreRow as ScoreRowImpl; // methods via impl
use sqlx::PgPool;

async fn example(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Insert a score
    let row = ScoreRow {
        user_id: 123,
        beatmap_id: 42,
        score_metadata_id: 1,
        replay_id: Some(5),
        rate: 1.0,
        hwid: "abc".into(),
        mods: serde_json::json!([]),
        hash: "SOMEHASH".into(),
        rank: "S".into(),
        status: "pending".into(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    let inserted = row.insert(pool).await?; // returns full row

    // Queries
    let by_id = ScoreRowImpl::find_by_id(pool, inserted.id).await?;
    let exists = ScoreRowImpl::exists_by_hash(pool, &inserted.hash).await?;
    let _ = ScoreRowImpl::update_status(pool, inserted.id, "validated").await?;
    Ok(())
}
```

## Other available domains (similar API)

- `models::beatmap` / `models::beatmapset`
- `models::pending_beatmap`
- `models::replay`
- `models::score_metadata`
- `models::score_rating`
- `models::msd`

## Tracing and logging

The crate depends on `tracing` and `tracing-subscriber`, but does not initialize subscribers for you. In your binary:

```rust 
fn main(){
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();
}
```

## Notes

- This library expects existing tables with matching schemas.
- All queries use `sqlx` (runtime: Tokio + rustls). Ensure Tokio is running in your app.
- Validation helpers live in `validators.rs` per model and `utils.rs` for common regexes.

