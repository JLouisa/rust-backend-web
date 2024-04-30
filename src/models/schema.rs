use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Sqlite, SqlitePool};
use std::result::Result;

pub async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(&db_url).await?;
    let qry = "PRAGMA foreign_keys = ON ;
    CREATE TABLE IF NOT EXISTS users
        (
            user_id                 TEXT PRIMARY KEY NOT NULL,
            username                TEXT UNIQUE NOT NULL,
            hashed_password         TEXT NOT NULL,
            created_on              DATETIME DEFAULT (datetime('now','localtime')),
            updated_on              DATETIME DEFAULT (datetime('now','localtime')),
            active                  BOOLEAN NOT NULL DEFAULT 1
        );";
    let result = sqlx::query(&qry).execute(&pool).await;
    pool.close().await;
    return result;
}
