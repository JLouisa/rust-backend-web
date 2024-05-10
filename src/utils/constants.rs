use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::domain::shops::Shop;

#[macro_export]
macro_rules! load_settings {
    // Variant for only key
    ($key:expr) => {{
        dotenvy::dotenv().ok(); // Load .env file if present
        std::env::var($key).expect(&format!("{} should be set", $key))
    }};
    // Variant for key with a default value
    ($key:expr, $default:expr) => {{
        dotenvy::dotenv().ok(); // Load .env file if present
        std::env::var($key).unwrap_or_else(|_| $default.to_string())
    }};
    // Variant for key with a default value that's an u16
    ($key:expr, $default:expr, u16) => {{
        dotenvy::dotenv().ok(); // Load .env file if present
        std::env::var($key)
            .unwrap_or_else(|_| $default.to_string())
            .parse::<u16>()
            .expect("Expected a number")
    }};
}

lazy_static! {
    // Setup Server Constants
    pub static ref ADDRESS: String = load_settings!("ADDRESS", "127.0.0.1");
    pub static ref PORT: u16 = load_settings!("PORT", 3000).parse().expect("Port is not a number");
    // Setup Database Constants
    pub static ref DATABASE_URL: String = load_settings!("DATABASE_URL");
    pub static ref DATABASE_SQLITE_URL: String = load_settings!("DATABASE_SQLITE_URL");
    // Setup Token Constants
    pub static ref TOKEN_SECRET: String = load_settings!("TOKEN_SECRET");
    pub static ref TOKEN_SK: String = load_settings!("TOKEN_SK");
    // Setup Email Constants
    pub static ref SMTP_HOST: String = load_settings!("SMTP_HOST");
    pub static ref EMAIL_HOST: String = load_settings!("EMAIL_HOST");
    pub static ref EMAIL_PASSWORD: String = load_settings!("EMAIL_PASSWORD");
    // Setup Redis Constants
    pub static ref REDIS_URL: String = load_settings!("REDIS_URL");
    // AWS S3 Constants
    pub static ref UPLOAD_SERVICE: String = load_settings!("UPLOAD_SERVICE");
    pub static ref AWS_ACCESS_KEY_ID: String = load_settings!("AWS_ACCESS_KEY_ID");
    pub static ref AWS_ACCESS_SECRET_KEY: String = load_settings!("AWS_ACCESS_SECRET_KEY");
    pub static ref AWS_REGION: String = load_settings!("AWS_REGION");
    pub static ref AWS_BUCKET_NAME: String = load_settings!("AWS_BUCKET_NAME");
    // Stripe Constants
    pub static ref STRIPE_SECRET: String = load_settings!("STRIPE_SECRET");
    pub static ref STRIPE_WEBHOOK_SECRET: String = load_settings!("STRIPE_WEBHOOK_SECRET");
    // Setup Shop Configurations
    pub static ref SHOP_CONFIGS: Mutex<HashMap<String, Shop>> = Mutex::new(HashMap::new());
}

pub struct EmailSettings {
    pub host: String,
    pub email: String,
    pub password: String,
}

pub fn get_email_settings() -> EmailSettings {
    EmailSettings {
        host: SMTP_HOST.clone(),
        email: EMAIL_HOST.clone(),
        password: EMAIL_PASSWORD.clone(),
    }
}

pub struct Config {
    pub address: String,
    pub port: u16,
    pub sqlx_database_url: String,
    pub redis_url: String,
}

impl Config {
    pub fn load_configuration() -> Self {
        dotenvy::dotenv().expect(".env file not found");

        Config {
            address: ADDRESS.clone(),
            port: PORT.clone(),
            sqlx_database_url: DATABASE_SQLITE_URL.clone(),
            redis_url: REDIS_URL.clone(),
        }
    }
}
