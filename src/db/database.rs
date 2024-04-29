// crates
use actix_web::*;
use diesel::result::Error as DieselError;
use diesel::PgConnection;

// Database setup
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

// Files
use crate::models::user_model::User;
use crate::schema::users::dsl::*;
use crate::utils;

pub type DBpool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pub pool: DBpool,
}

impl Database {
    // Create Database Pool
    pub fn new() -> Self {
        dotenv().expect(".env file not found");

        let database_url: String = utils::constants::DATABASE_URL.clone();
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let result = r2d2::Pool::builder()
            .max_size(10)
            .build(manager)
            .expect("Failed to create Pool");

        Database { pool: result }
    }
    // GET One User
    pub fn get_one_user(&self, the_id: String) -> Option<User> {
        let user: Option<User> = users
            .find(the_id)
            .first::<User>(&mut self.pool.get().expect("Something wrong with pool"))
            .ok();

        return user;
    }

    // GET All Users
    pub fn get_all_users(&self) -> Result<Vec<User>, DieselError> {
        let the_users: Result<Vec<User>, DieselError> = users
            .select(User::as_select())
            .load(&mut self.pool.get().expect("Something wrong with pool"));

        return the_users;
    }

    // UPDATE One User
    pub fn update_one_user(&self, user: User) -> Result<User, DieselError> {
        let updated_user = diesel::update(users.filter(id.eq(&user.id)))
            .set(&user)
            .get_result(&mut self.pool.get().expect("Something wrong with pool"));

        return updated_user;
    }

    // DELETE One User
    pub fn delete_one_user(&self, the_id: String) -> Result<User, DieselError> {
        let get_one_user = self.get_one_user(the_id.clone());

        match get_one_user {
            Some(content) => {
                let one_user = diesel::delete(users.filter(id.eq(&the_id)))
                    .execute(&mut self.pool.get().expect("Something wrong with pool"));

                match one_user {
                    Ok(_) => return Ok(content),
                    Err(err) => return Err(err),
                }
            }
            None => Err(DieselError::NotFound),
        }
    }
}
