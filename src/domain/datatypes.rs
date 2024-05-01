use crate::modules::password_hash;
use actix_web::cookie::time;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserServer {
    pub user_id: String,
    pub username: String,
    pub hashed_password: String,
    pub active: bool,
}
impl UserServer {
    pub fn process_for_server(user_client_in: UserClientIn) -> Self {
        let user_id: String = format!("{:?}", Uuid::new_v4());
        let user_active: bool = true;
        let password = password_hash::Password::hash_password(user_client_in.password.as_str())
            .expect("Error hashing the password");

        return UserServer {
            user_id,
            username: user_client_in.username.to_string(),
            hashed_password: password.get_password_string(),
            active: user_active,
        };
    }

    pub fn process_for_client(&self) -> UserClientOut {
        return UserClientOut {
            user_id: self.user_id.to_string(),
            username: self.username.to_string(),
            hashed_password: self.hashed_password.to_string(),
            active: self.active,
        };
    }
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserClientSignIn {
    pub username: String,
    pub password: String,
    pub remember: Option<bool>,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserClientIn {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserClientOut {
    pub user_id: String,
    pub username: String,
    pub hashed_password: String,
    pub active: bool,
}

pub enum LoginTypes {
    Succesfull,
    Failed,
}

pub enum CookieVariations {
    Auth,
    ShoppingCarts,
    Personalization,
    Payment,
    NAW,
}

pub struct Settings {
    pub value: String,
    pub time: time::OffsetDateTime,
}
impl Settings {
    pub fn new(value: &str, remember: bool) -> Self {
        Settings {
            value: value.to_string(),
            time: match remember {
                true => time::OffsetDateTime::now_utc() + time::Duration::days(7),
                false => time::OffsetDateTime::now_utc() + time::Duration::days(1),
            },
        }
    }
}
