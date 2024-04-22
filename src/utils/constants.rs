use lazy_static::lazy_static;

lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_database_url();
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
