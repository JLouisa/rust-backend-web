use crate::{controllers, view};
use actix_web::*;

// this function could be located in a different module
pub fn root_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(root::index_page)
            .service(root::endpoints_page)
            // .service(root::login_page)
            .service(root::echo)
            .service(root::hello)
            .service(root::json_post),
    );
}

// Root Routes Handlers (Controller)
pub mod root {
    use super::*;

    // Index
    #[get("/")]
    pub async fn index_page() -> impl Responder {
        let mut context = tera::Context::new();
        context.insert("home_msg_from_rust", "Msg from Rust server");
        context.insert("ping_pong", "ping");

        match view::setup::TEMPLATES.render("pages/index/index.html", &context) {
            Ok(content) => return HttpResponse::Ok().body(content),
            Err(err) => {
                eprintln!("Error rendering index page: {}", err);
                return HttpResponse::InternalServerError().finish(); // Return 500 Internal Server Error
            }
        };
    }

    #[get("/endpoints")]
    pub async fn endpoints_page() -> impl Responder {
        let mut context = tera::Context::new();
        context.insert("msg_from_rust", "Msg from Rust server");
        context.insert("ping_pong", "ping");

        match view::setup::TEMPLATES.render("pages/endpoints/endpoints.html", &context) {
            Ok(content) => return HttpResponse::Ok().body(content),
            Err(err) => {
                eprintln!("Error rendering index page: {}", err);
                return HttpResponse::InternalServerError().finish(); // Return 500 Internal Server Error
            }
        };
    }

    // #[get("/login")]
    // pub async fn login_page() -> HttpResponse {
    //     let mut context = tera::Context::new();

    //     context.insert("login_msg", "Please login to continue");
    //     match view::setup::TEMPLATES.render("pages/login/login.html", &context) {
    //         Ok(content) => return HttpResponse::Ok().body(content),
    //         Err(err) => {
    //             eprintln!("Error rendering index page: {}", err);
    //             return HttpResponse::InternalServerError().finish(); // Return 500 Internal Server Error
    //         }
    //     }
    // }

    //Hello
    #[get("/hello")]
    pub async fn hello() -> impl Responder {
        controllers::user::index::get_hello()
    }

    //Echo
    #[post("/echo")]
    pub async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }

    //POST JSON
    type TheUser = controllers::user::json::User2;
    #[post("/json")]
    pub async fn json_post(item: web::Json<TheUser>) -> impl Responder {
        controllers::user::json::json_post(item)
    }
}
