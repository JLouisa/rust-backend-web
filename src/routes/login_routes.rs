use crate::controllers;
use crate::db::sqlite::SqliteDB;
use crate::domain::datatypes::{UserClientIn, UserServer};
use actix_web::*;

// this function could be located in a different module
pub fn login_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/login")
            .service(login::test)
            .service(login::post_login),
    );
}

pub mod login {
    use super::*;

    #[get("/test")]
    pub async fn test() -> impl Responder {
        HttpResponse::Ok().body("GET Login")
    }

    #[post("")]
    pub async fn post_login(
        db: web::Data<SqliteDB>,
        login_info: web::Json<UserClientIn>,
    ) -> impl Responder {
        let user = login_info.into_inner();
        controllers::login::verify_login(db, user).await
    }
}
