// Files
use crate::models::user_model::User;
use crate::schema::users::dsl::*;

// crates
use actix_web::*;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

pub mod users_db {
    use super::*;

    pub fn get_all_users(connection: &mut PgConnection) -> Result<Vec<User>, DieselError> {
        let the_users: Result<Vec<User>, DieselError> =
            users.select(User::as_select()).load(connection);
        return the_users;
    }

    pub fn get_one_user(
        the_id: &String,
        connection: &mut PgConnection,
    ) -> Result<User, DieselError> {
        let user: Result<Vec<User>, DieselError> = users
            .filter(id.eq(&the_id))
            .select(User::as_select())
            .load(connection);

        match user {
            Ok(content) => Ok(content.get(0).expect("Should have Users").to_owned()),
            Err(err) => Err(err),
        }
    }

    pub fn delete_one_user(
        the_id: String,
        connection: &mut PgConnection,
    ) -> Result<User, DieselError> {
        let get_one_user = get_one_user(&the_id, connection);

        match get_one_user {
            Ok(content) => {
                let one_user = diesel::delete(users.filter(id.eq(&the_id))).execute(connection);

                match one_user {
                    Ok(_) => return Ok(content),
                    Err(err) => return Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }
}
