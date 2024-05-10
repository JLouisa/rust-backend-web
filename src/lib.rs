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
    pub mod shops;
    pub mod user_domain;
}

pub mod routes {
    pub mod app_routes;
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
    pub mod aws_s3;
    pub mod cookie;
    pub mod cuid;
    pub mod email;
    pub mod middleware;
    pub mod middleware_domain;
    pub mod middleware_msg;
    pub mod password_hash;
    pub mod pdf;
    pub mod redis;
    pub mod stripe {
        pub mod stripe;
        pub mod stripe_webhooks;
    }
    pub mod token_pub;
}

pub mod utils {
    pub mod constants;
}

pub mod view {
    pub mod setup;
}

pub mod schema;
