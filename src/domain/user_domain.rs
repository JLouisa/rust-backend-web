use crate::models::user_model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// User Client
#[derive(Serialize, Deserialize, Debug)]
pub struct UserClient {
    pub id: String,
    pub username: String,
}

// All User Client
#[derive(Serialize, Deserialize, Debug)]
pub struct AllUserClient {
    pub users: Vec<UserClient>,
}

// User
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    username: String,
    password: String,
}
impl User {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
    pub fn convert(self) -> user_model::User {
        let id = Uuid::new_v4();
        let user = user_model::User {
            id: format!("{}", id),
            username: self.username,
            hashed_password: self.password,
        };
        return user;
    }
    pub fn client(user: &user_model::User) -> UserClient {
        let client_user = UserClient {
            id: user.id.to_string(),
            username: user.username.to_string(),
        };
        return client_user;
    }
}
