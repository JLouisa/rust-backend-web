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

        return UserServer {
            user_id,
            username: user_client_in.username.to_string(),
            hashed_password: user_client_in.password,
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
pub struct UserClientIn {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserClientOut {
    pub user_id: String,
    pub username: String,
    pub hashed_password: String,
    pub active: bool,
}
