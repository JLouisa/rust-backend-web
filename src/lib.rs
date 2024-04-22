pub mod routes {
    pub mod ui_routes {
        pub mod index_page;
    }
    pub mod app_routes;
    pub mod root_routes;
    pub mod users_routes;
}

pub mod model {
    pub mod user_model;
}

pub mod controller {
    pub mod user;
}

pub mod utils {
    pub mod constants;
}

pub mod view {
    pub mod setup;
}

pub mod schema;
