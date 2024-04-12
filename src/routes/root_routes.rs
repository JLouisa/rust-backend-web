// use crate::*;
use actix_web::*;

// this function could be located in a different module
pub fn root_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(root::index)
            .service(root::echo)
            .service(root::hello)
            .service(root::json_get)
            .service(root::json_post),
    );
}

pub mod root {
    // use crate::*;
    use crate::controller;
    use actix_web::*;

    //Root
    #[get("")]
    pub async fn index() -> impl Responder {
        controller::user::index::get_homepage()
    }

    //Hello
    #[get("/hello")]
    pub async fn hello() -> impl Responder {
        controller::user::index::get_hello()
    }

    //Echo
    #[post("/echo")]
    pub async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }

    //GET JSON
    #[get("/json")]
    pub async fn json_get() -> impl Responder {
        controller::user::json::json_get()
    }

    //POST JSON
    type TheUser = controller::user::json::User;
    #[post("/json")]
    pub async fn json_post(item: web::Json<TheUser>) -> impl Responder {
        controller::user::json::json_post(item)
    }
}
