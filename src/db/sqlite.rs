use actix_web::*;
use sqlx::{self, sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::domain::{datatypes::UserServer, shops::ShopConfig};
use crate::models::queries;

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
    // Create Database Pool
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

    // Return Pool
    pub fn get_pool(&self) -> Pool<Sqlite> {
        return self.db.clone();
    }

    pub async fn get_one_user(&self, user_id: &str) -> Result<UserServer, sqlx::Error> {
        // Use the enum to get the query string
        let sql = queries::UserQueries::GetOneUser.convert_to_str();

        // Execute the query and return the result
        let result = sqlx::query_as::<_, UserServer>(sql)
            .bind(user_id)
            .fetch_one(&self.db)
            .await;

        println!("Query result: {:?}", result); // Debug print the result or error
        result
    }

    // GET One User with Username
    pub async fn get_one_user_username(
        &self,
        username: &str,
    ) -> Result<Option<UserServer>, sqlx::Error> {
        // SQL query select one user from the database using id
        let sql = queries::UserQueries::GetOneUserWithUsername.convert_to_str();

        return sqlx::query_as::<_, UserServer>(sql)
            .bind(username)
            .fetch_optional(&self.db)
            .await;
    }

    // GET All Users
    pub async fn get_all_users(&self) -> Result<Vec<UserServer>, sqlx::Error> {
        // SQL query select one user from the database using id
        let sql = queries::UserQueries::GetAllUsers.convert_to_str();

        return sqlx::query_as::<_, UserServer>(sql)
            .fetch_all(&self.db)
            .await;
    }

    // POST One User
    pub async fn create_one_user(&self, user: &UserServer) -> Result<UserServer, sqlx::Error> {
        // SQL query to insert the user into the database and return the inserted user
        let sql = queries::UserQueries::CreateOneUser.convert_to_str();

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

    // PUT One User
    pub async fn update_one_user(&self, user: &UserServer) -> Result<UserServer, sqlx::Error> {
        // SQL query to insert the user into the database and return the inserted user
        let sql = queries::UserQueries::UpdateOneUser.convert_to_str();

        match sqlx::query(sql)
            .bind(&user.username)
            .bind(&user.hashed_password)
            .bind(&user.active)
            .bind(&user.user_id)
            .execute(&self.db)
            .await
        {
            Ok(_) => {
                // Await the result of get_one_user before returning it
                return self.get_one_user(&user.user_id).await;
            }
            Err(err) => {
                eprintln!("Error updating user: {:?}", err);
                Err(err)
            }
        }
    }

    // PUT One User Password
    pub async fn update_one_user_password(
        &self,
        user: &UserServer,
    ) -> Result<UserServer, sqlx::Error> {
        // SQL query to insert the user into the database and return the inserted user
        let sql = queries::UserQueries::UpdateOneUserPwd.convert_to_str();

        match sqlx::query(sql)
            .bind(&user.hashed_password)
            .bind(&user.user_id)
            .execute(&self.db)
            .await
        {
            Ok(_) => {
                // Await the result of get_one_user before returning it
                return self.get_one_user(user.user_id.as_str()).await;
            }
            Err(err) => {
                eprintln!("Error updating user: {:?}", err);
                Err(err)
            }
        }
    }

    // DELETE One User
    pub async fn delete_one_user(&self, user_id: &str) -> Result<String, sqlx::Error> {
        let sql = queries::UserQueries::DeleteOneUser.convert_to_str();

        match sqlx::query(sql).bind(user_id).execute(&self.db).await {
            Ok(_) => return Ok("succesfully".to_string()),
            Err(err) => {
                eprintln!("Error deleting user: {:?}", err);
                return Err(err);
            }
        }
    }

    // Transaction
    pub async fn transaction(&self, user: &UserServer) -> Result<UserServer, sqlx::Error> {
        // Start a new transaction
        let mut txn = self.db.begin().await?;

        // Define SQL queries
        let create_sql = queries::UserQueries::CreateOneUser.convert_to_str();
        let update_sql = queries::UserQueries::UpdateOneUser.convert_to_str();

        // Execute the first query to create a new user
        sqlx::query(create_sql)
            .bind(&user.user_id)
            .bind(&user.username)
            .bind(&user.hashed_password)
            .bind(&user.active)
            .execute(&mut *txn)
            .await?;

        // Execute the second query to update the user
        sqlx::query(update_sql)
            .bind("TXN_Username")
            .bind("TXN_Password")
            .bind(false)
            .bind(&user.user_id)
            .execute(&mut *txn)
            .await?;

        // Commit the transaction
        txn.commit().await?;

        // Fetch the updated user from the database
        return self.get_one_user(&user.user_id).await;
    }

    // GET All Shop Domains
    pub async fn get_all_shop_domains(&self) -> Result<Vec<ShopConfig>, sqlx::Error> {
        // SQL query select one user from the database using id
        let sql = queries::ShopQueries::GetAllShops.convert_to_str();

        return sqlx::query_as::<_, ShopConfig>(sql)
            .fetch_all(&self.db)
            .await;
    }
    // GET One Shop Domain
    pub async fn get_one_shop_domain(&self, shop: &str) -> Result<ShopConfig, sqlx::Error> {
        // SQL query select one user from the database using id
        let sql = queries::ShopQueries::GetOneShop.convert_to_str();

        return sqlx::query_as::<_, ShopConfig>(sql)
            .bind(shop)
            .fetch_one(&self.db)
            .await;
    }
}
