pub mod controllers {
    pub mod ui_controller {
        pub mod index;
    }
    pub mod user;
}

pub mod db {
    pub mod database;
    pub mod db_setup;
}

pub mod domain {
    pub mod user_domain;
}

pub mod routes {
    pub mod app_routes;
    pub mod root_routes;
    pub mod ui_routes;
    pub mod users_routes;
}

pub mod models {
    pub mod user_model;
}

pub mod utils {
    pub mod constants;
}

pub mod view {
    pub mod setup;
}

pub mod schema;
