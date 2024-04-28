use crate::controllers::ui_controller::index::*;
use crate::db::db_setup::establish_connection;
use actix_web::*;

// this function could be located in a different module
pub fn ui_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/ui")
            .service(index::hello)
            .service(index::ping_pong)
            .service(index::show_all_user_list)
            .service(index::delete_one_user),
    );
}

// Index Routes Handlers (Controller)
pub mod index {
    use super::*;

    #[get("/index/hello")]
    pub async fn hello() -> impl Responder {
        return index::index_ui_controller::hello();
    }

    #[get("/index/mirror/{ping_pong}")]
    pub async fn ping_pong(path: web::Path<String>) -> impl Responder {
        let ping_pong: String = path.into_inner();
        println!("ping_pong: {}", ping_pong);

        return index::index_ui_controller::ping_pong(ping_pong);
    }

    #[get("/index/show/users")]
    pub async fn show_all_user_list() -> impl Responder {
        let connection = &mut establish_connection();

        return index::index_ui_controller::show_all_user_list(connection);
    }

    #[delete("/index/delete/{id}")]
    pub async fn delete_one_user(path: web::Path<String>) -> impl Responder {
        let user_id: String = path.into_inner();
        let connection = &mut establish_connection();

        return index::index_ui_controller::deleted_user(user_id, connection);
    }
}
