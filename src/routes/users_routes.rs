use crate::controllers;
use crate::db::diesel::Database;
use crate::domain::user_domain;
use actix_web::*;

// this function could be located in a different module
pub fn users_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/users")
            .service(user::get_all_user)
            .service(user::get_one_user)
            .service(user::post_one_user)
            .service(user::put_one_user)
            .service(user::delete_one_user),
    );
}

// User Routes Handlers (Controller)
pub mod user {
    use super::*;

    // GET all Users
    #[get("")]
    pub async fn get_all_user(db: web::Data<Database>) -> impl Responder {
        controllers::ui_controller::index::index_ui_controller::show_all_user_list_diesel(db)
    }

    // GET One User
    #[get("/{id}")]
    pub async fn get_one_user(user_id: web::Path<String>, db: web::Data<Database>) -> HttpResponse {
        let user_id = user_id.into_inner();
        controllers::user::user::get_one_user(user_id, db)
    }

    // Create One User
    #[post("/create")]
    pub async fn post_one_user(user: web::Json<user_domain::User>) -> HttpResponse {
        let user = user.into_inner().convert();
        controllers::user::user::post_one_user(user)
    }

    // Update One User
    #[put("/create")]
    pub async fn put_one_user(user: web::Json<user_domain::UserClient>) -> HttpResponse {
        let user = user.into_inner();
        controllers::user::user::put_one_user(user)
    }

    // DELETE One User
    #[delete("/{id}")]
    pub async fn delete_one_user(path: web::Path<String>) -> HttpResponse {
        let user_id = path.into_inner();

        controllers::user::user::delete_one_user(user_id)
    }
}
