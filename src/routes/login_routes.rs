use crate::db::sqlite::SqliteDB;
use crate::domain::datatypes::{UserClientIn, UserClientSignIn};
use crate::{controllers, view};
use actix_web::*;

// this function could be located in a different module
pub fn login_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/login")
            .service(login::test)
            .service(login::login_page)
            .service(login::post_login),
    );
}

pub mod login {
    use super::*;

    #[get("/test")]
    pub async fn test() -> impl Responder {
        HttpResponse::Ok().body("GET Login")
    }

    #[get("")]
    pub async fn login_page() -> HttpResponse {
        let mut context = tera::Context::new();

        context.insert("login_msg", "Please login to continue");
        context.insert("login_value_username", "");
        context.insert("login_value_password", "");
        context.insert("login_failed_msg", "");
        match view::setup::TEMPLATES.render("pages/login/login.html", &context) {
            Ok(content) => return HttpResponse::Ok().body(content),
            Err(err) => {
                eprintln!("Error rendering index page: {}", err);
                return HttpResponse::InternalServerError().finish(); // Return 500 Internal Server Error
            }
        }
    }
    // POST Login info with remember field optional
    #[post("")]
    pub async fn post_login(
        db: web::Data<SqliteDB>,
        login_info: web::Form<UserClientSignIn>,
    ) -> impl Responder {
        let user = login_info.into_inner();

        controllers::login::verify_login(db, user).await
    }
}
