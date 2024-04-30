use derive_more::{Display, From};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use actix_web::*;
use sqlx::Row;
use sqlx::{self, FromRow};
use uuid::Uuid;

use crate::domain::datatypes::UserClientOut;
use crate::domain::{datatypes::UserServer, user_domain::User};

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Debug, Clone)]
pub struct SqliteDB {
    pub db: Pool<Sqlite>,
}

impl SqliteDB {
    pub async fn new(db_sqlite_url: &str) -> Self {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_sqlite_url)
            .await;

        let pool = match pool {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to connect to database: {}", e);
                eprintln!(
                    "If the database has not been created, please run \n $ sqlx database setup \n"
                );
                panic!("Database connection failed");
            }
        };

        return SqliteDB { db: pool };
    }

    pub fn get_pool(&self) -> Pool<Sqlite> {
        return self.db.clone();
    }

    pub async fn get_one_user(&self, user_id: &String) -> Result<UserServer, sqlx::Error> {
        // SQL query select one user from the database using id
        let sql = "SELECT * FROM users WHERE user_id = ?";

        return sqlx::query_as::<_, UserServer>(sql)
            .bind(user_id)
            .fetch_one(&self.db)
            .await;
    }

    pub async fn get_all_user(&self) -> Result<Vec<UserServer>, sqlx::Error> {
        // SQL query select one user from the database using id
        let sql = "SELECT * FROM users";

        return sqlx::query_as::<_, UserServer>(sql)
            .fetch_all(&self.db)
            .await;
    }

    pub async fn create_one_user(&self, user: &UserServer) -> Result<UserServer, sqlx::Error> {
        // SQL query to insert the user into the database and return the inserted user
        let sql =
            "INSERT INTO users (user_id, username, hashed_password, active) VALUES (?, ?, ?, ?)";

        match sqlx::query(sql)
            .bind(&user.user_id)
            .bind(&user.username)
            .bind(&user.hashed_password)
            .bind(&user.active)
            .execute(&self.db)
            .await
        {
            Ok(_) => {
                // Await the result of get_one_user before returning it
                match self.get_one_user(&user.user_id).await {
                    Ok(created_user) => Ok(created_user),
                    Err(err) => {
                        eprintln!("Error retrieving created user: {:?}", err);
                        Err(err)
                    }
                }
            }
            Err(err) => {
                eprintln!("Error creating user: {:?}", err);
                Err(err)
            }
        }
    }
}
