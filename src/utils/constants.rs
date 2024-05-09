use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::domain::shops::Shop;

lazy_static! {
    // Setup Server Constants
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    // Setup Database Constants
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref DATABASE_SQLITE_URL: String = set_database_sqlite_url();
    // Setup Token Constants
    pub static ref TOKEN_SECRET: String = set_token_secret();
    pub static ref TOKEN_SK: String = set_token_sk();
    // Setup Shop Configurations
    pub static ref SHOP_CONFIGS: Mutex<HashMap<String, Shop>> = Mutex::new(HashMap::new());
    // Setup Email Constants
    pub static ref SMTP_HOST: String = set_smtp_host();
    pub static ref EMAIL_HOST: String = set_email_host();
    pub static ref EMAIL_PASSWORD: String = set_email_password();
    // Setup Redis Constants
    pub static ref REDIS_URL: String = set_redis_url();
    // AWS S3 Constants
    pub static ref AWS_S3_BUCKET: String = set_aws_s3_bucket();
    pub static ref AWS_S3_REGION: String = set_aws_s3_region();
    // Stripe Constants
    pub static ref STRIPE_SECRET: String = set_stripe_secret();
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

// Get the address from the .env file
fn set_address() -> String {
    dotenvy::dotenv().ok();
    let address: String = match std::env::var("ADDRESS") {
        Ok(the_address) => the_address.parse().expect("ADDRESS should be a string"),
        Err(_) => "127.0.0.1".to_string(),
    };
    return address;
}

// Get the port from the .env file
fn set_port() -> u16 {
    dotenvy::dotenv().ok();
    let port: u16 = match std::env::var("PORT") {
        Ok(the_port) => the_port.parse::<u16>().expect("PORT should be a number"),
        Err(_) => 3000,
    };
    return port;
}

fn set_database_url() -> String {
    dotenvy::dotenv().ok();
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return database_url;
}

fn set_database_sqlite_url() -> String {
    dotenvy::dotenv().ok();
    let database_url: String =
        std::env::var("DATABASE_SQLITE_URL").expect("DATABASE_SQLITE_URL must be set");
    return database_url;
}

fn set_token_secret() -> String {
    dotenvy::dotenv().ok();
    let token_secret: String = std::env::var("TOKEN_SECRET").expect("TOKEN_SECRET must be set");
    return token_secret;
}

fn set_token_sk() -> String {
    dotenvy::dotenv().ok();
    let token_secret: String = std::env::var("TOKEN_SK").expect("TOKEN_SK must be set");
    return token_secret;
}

fn set_smtp_host() -> String {
    dotenvy::dotenv().ok();
    let smtp_host: String = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");
    return smtp_host;
}

fn set_email_host() -> String {
    dotenvy::dotenv().ok();
    let email_host: String = std::env::var("EMAIL_HOST").expect("EMAIL_HOST must be set");
    return email_host;
}

fn set_email_password() -> String {
    dotenvy::dotenv().ok();
    let email_password: String =
        std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD must be set");
    return email_password;
}

fn set_redis_url() -> String {
    dotenvy::dotenv().ok();
    let redis_url: String = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    return redis_url;
}

fn set_aws_s3_bucket() -> String {
    dotenvy::dotenv().ok();
    let aws_s3_bucket: String = std::env::var("AWS_S3_BUCKET").expect("AWS_S3_BUCKET must be set");
    return aws_s3_bucket;
}

fn set_aws_s3_region() -> String {
    dotenvy::dotenv().ok();
    let aws_s3_region: String = std::env::var("AWS_S3_REGION").expect("AWS_S3_REGION must be set");
    return aws_s3_region;
}

fn set_stripe_secret() -> String {
    dotenvy::dotenv().ok();
    let stripe_secret: String = std::env::var("STRIPE_SECRET").expect("STRIPE_SECRET must be set");
    return stripe_secret;
}
