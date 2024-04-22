// use crate::*;
use actix_web::*;

// this function could be located in a different module
pub fn users_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/users")
            .service(user::get_all_user)
            .service(user::get_one_user)
            .service(user::post_one_user),
    );
}

pub mod user {
    use crate::controllers;
    use actix_web::*;

    //Users
    #[get("")]
    pub async fn get_all_user() -> impl Responder {
        controllers::user::user::get_all()
    }
    #[get("/{id}")]
    pub async fn get_one_user(path: web::Path<(String,)>) -> HttpResponse {
        controllers::user::user::get_one(path)
    }

    // POST Users
    #[post("")]
    pub async fn post_one_user() -> HttpResponse {
        controllers::user::user::post_one()
    }
}
