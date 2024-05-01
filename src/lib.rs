pub mod controllers {
    pub mod ui_controller {
        pub mod index;
        pub mod login;
    }
    pub mod login;
    pub mod user;
}

pub mod db {
    pub mod db_setup;
    pub mod diesel;
    pub mod sqlite;
}

pub mod domain {
    pub mod datatypes;
    pub mod user_domain;
}

pub mod routes {
    pub mod app_routes;
    pub mod login_routes;
    pub mod root_routes;
    pub mod ui_routes;
    pub mod users_routes;
}

pub mod models {
    pub mod queries;
    pub mod schema;
    pub mod user_model;
}

pub mod modules {
    pub mod cookie;
    pub mod password_hash;
    pub mod token_pub;
}

pub mod utils {
    pub mod constants;
}

pub mod view {
    pub mod setup;
}

pub mod schema;
