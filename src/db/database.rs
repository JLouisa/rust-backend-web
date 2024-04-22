// Files
use crate::db::db_setup::establish_connection;
use crate::domain::user_domain;
use crate::models::{self, user_model::User};
use crate::schema::users::dsl::*;

// crates
use actix_web::*;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::PgConnection;
use diesel::{delete, insert_into, update};
use serde::{Deserialize, Serialize};

pub mod users_db {
    use super::*;

    pub fn get_all_users(connection: &mut PgConnection) -> Result<Vec<User>, DieselError> {
        let the_users: Result<Vec<User>, DieselError> =
            users.select(User::as_select()).load(connection);
        return the_users;
    }
}
