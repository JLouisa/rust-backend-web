use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::domain::shops::Shop;

lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref DATABASE_SQLITE_URL: String = set_database_sqlite_url();
    pub static ref TOKEN_SECRET: String = set_token_secret();
    pub static ref TOKEN_SK: String = set_token_sk();
    pub static ref SHOP_CONFIGS: Mutex<HashMap<String, Shop>> = Mutex::new(HashMap::new());
}

// Get the address from the .env file
fn set_address() -> String {
    dotenv::dotenv().ok();
    let address: String = match std::env::var("ADDRESS") {
        Ok(the_address) => the_address.parse().expect("ADDRESS should be a string"),
        Err(_) => "127.0.0.1".to_string(),
    };
    return address;
}

// Get the port from the .env file
fn set_port() -> u16 {
    dotenv::dotenv().ok();
    let port: u16 = match std::env::var("PORT") {
        Ok(the_port) => the_port.parse::<u16>().expect("PORT should be a number"),
        Err(_) => 3000,
    };
    return port;
}

fn set_database_url() -> String {
    dotenv::dotenv().ok();
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return database_url;
}

fn set_database_sqlite_url() -> String {
    dotenv::dotenv().ok();
    let database_url: String =
        std::env::var("DATABASE_SQLITE_URL").expect("DATABASE_SQLITE_URL must be set");
    return database_url;
}

fn set_token_secret() -> String {
    dotenv::dotenv().ok();
    let token_secret: String = std::env::var("TOKEN_SECRET").expect("TOKEN_SECRET must be set");
    return token_secret;
}

fn set_token_sk() -> String {
    dotenv::dotenv().ok();
    let token_secret: String = std::env::var("TOKEN_SK").expect("TOKEN_SK must be set");
    return token_secret;
}
