use sqlx::{sqlite::SqliteQueryResult, SqlitePool};
use std::result::Result;

pub async fn create_schema(db_url: &str) -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect(db_url).await?;
    // Enable foreign key constraints
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await?;

    // Create users table
    let users_table_query = "
        CREATE TABLE IF NOT EXISTS users
        (
            user_id                 TEXT PRIMARY KEY NOT NULL,
            username                TEXT UNIQUE NOT NULL,
            hashed_password         TEXT NOT NULL,
            created_on              DATETIME DEFAULT (datetime('now','localtime')),
            updated_on              DATETIME DEFAULT (datetime('now','localtime')),
            active                  BOOLEAN NOT NULL DEFAULT 1
        );";
    sqlx::query(users_table_query).execute(&pool).await?;
    println!("User table created.");

    // Create shop_configurations table
    let shop_configurations_query = "
        CREATE TABLE IF NOT EXISTS shop_configurations
        (
            domain             TEXT PRIMARY KEY NOT NULL,
            name               TEXT NOT NULL,
            product_type       TEXT NOT NULL
        );";
    sqlx::query(shop_configurations_query)
        .execute(&pool)
        .await?;
    println!("Shop_configuration table created.");

    // Create shop_configurations table
    let products_query = "
        CREATE TABLE IF NOT EXISTS products
        (
            product_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            price DECIMAL NOT NULL,
            in_stock BOOLEAN DEFAULT TRUE
        );";
    sqlx::query(products_query).execute(&pool).await?;
    println!("products table created.");

    pool.close().await;
    Ok(())
}
