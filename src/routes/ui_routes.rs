use crate::controllers::ui_controller::*;
use crate::db::diesel::Database;
use actix_web::*;

// this function could be located in a different module
pub fn ui_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/ui")
            .service(index_ui::hello)
            .service(index_ui::ping_pong)
            .service(index_ui::show_all_user_list)
            .service(index_ui::delete_one_user)
            .service(login_ui::get_login),
    );
}

// Index Routes Handlers (Controller)
pub mod index_ui {
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
    pub async fn show_all_user_list(db: web::Data<Database>) -> impl Responder {
        return index::index_ui_controller::show_all_user_list_diesel(db);
    }

    #[delete("/index/delete/{id}")]
    pub async fn delete_one_user(
        path: web::Path<String>,
        db: web::Data<Database>,
    ) -> impl Responder {
        let user_id: String = path.into_inner();

        return index::index_ui_controller::deleted_user(user_id, db);
    }
}

pub mod login_ui {
    use super::*;

    #[get("/login")]
    pub async fn get_login() -> impl Responder {
        return login::login_ui_controller::login_page();
    }
}
