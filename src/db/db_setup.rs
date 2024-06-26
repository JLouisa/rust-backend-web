use crate::utils;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url: String = utils::constants::DATABASE_URL.clone();

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
