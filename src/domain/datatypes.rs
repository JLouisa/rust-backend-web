use crate::modules::password_hash;
use actix_web::cookie::time;
use actix_web::cookie::Cookie;
use pasetors::claims::Claims;
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
pub struct UserClientRegister {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}
impl UserClientRegister {
    pub fn verify_password(&self) -> Result<UserServer, ()> {
        if self.password == self.confirm_password {
            let user = UserClientIn {
                username: self.username.to_string(),
                password: self.password.to_string(),
            };
            Ok(UserServer::process_for_server(user))
        } else {
            Err(())
        }
    }
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
#[derive(Serialize, Deserialize, Debug)]
pub struct UserCookie {
    pub user_id: String,
    pub username: String,
}
impl UserCookie {
    pub fn new(cookie: &Claims) -> Self {
        UserCookie {
            user_id: cookie
                .get_claim("user_id")
                .expect("Failed to get user_id")
                .to_string(),
            username: cookie
                .get_claim("username")
                .expect("Failed to get user_id")
                .to_string(),
        }
    }
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
impl CookieVariations {
    pub fn get_name(&self) -> String {
        match self {
            CookieVariations::Auth => "auth".to_string(),
            CookieVariations::ShoppingCarts => "shopping_cart".to_string(),
            CookieVariations::Personalization => "personalization".to_string(),
            CookieVariations::Payment => "payment".to_string(),
            CookieVariations::NAW => "naw".to_string(),
        }
    }
    pub fn generate_cookie(&self, setting: Settings) -> Cookie {
        match self {
            &CookieVariations::Auth => Cookie::build(self.get_name(), setting.value)
                .path("/")
                // .domain(domain.to_string())
                .expires(setting.time.to_owned())
                .secure(true)
                .http_only(false)
                .finish(),
            &CookieVariations::ShoppingCarts => Cookie::build(self.get_name(), setting.value)
                .path("/")
                .expires(setting.time.to_owned())
                .secure(true)
                .http_only(false)
                .finish(),
            _ => todo!("Generate the rest of the cookies"),
        }
    }
    pub fn create_user_info(&self, cookie: &Claims) -> UserCookie {
        match self {
            &CookieVariations::Auth => UserCookie {
                user_id: cookie
                    .get_claim("user_id")
                    .expect("Failed to get user_id")
                    .to_string(),
                username: cookie
                    .get_claim("username")
                    .expect("Failed to get user_id")
                    .to_string(),
            },
            _ => todo!("Create the rest of the cookies"),
        }
    }
    pub fn remove_cookie(&self) -> Cookie {
        match self {
            &CookieVariations::Auth => Cookie::build(self.get_name(), "")
                .path("/")
                .expires(time::OffsetDateTime::now_utc())
                .max_age(time::Duration::seconds(0))
                .secure(true)
                .http_only(false)
                .finish(),
            &CookieVariations::ShoppingCarts => Cookie::build(self.get_name(), "")
                .path("/")
                .expires(time::OffsetDateTime::now_utc())
                .max_age(time::Duration::seconds(0))
                .secure(true)
                .http_only(false)
                .finish(),
            _ => todo!("Remove the rest of the cookies"),
        }
    }
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
