pub mod root {
    // use crate::*;
    use crate::controller;
    use actix_web::*;

    //Root
    #[get("/")]
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

    #[get("/json")]
    pub async fn json_get() -> impl Responder {
        controller::user::json::json_get()
    }
    // #[post("/json")]
    // pub async fn json_post(req_body: String) -> impl Responder {
    //     controller::user::json::json_post(req_body)
    // }
    type TheUser = controller::user::json::User;

    #[post("/json")]
    pub async fn json_post(item: web::Json<TheUser>) -> impl Responder {
        controller::user::json::json_post(item)
    }
}

pub mod user {
    use crate::controller;
    use actix_web::*;

    //Users
    #[get("")]
    pub async fn get_all_user() -> impl Responder {
        controller::user::user::get_all()
    }
    #[get("/{id}")]
    pub async fn get_one_user(path: web::Path<(String,)>) -> HttpResponse {
        controller::user::user::get_one(path)
    }

    // POST Users
    #[post("/{id}")]
    pub async fn post_one_user(path: web::Path<(String,)>) -> HttpResponse {
        controller::user::user::post_one(path)
    }
}
pub mod app {
    use actix_web::*;

    //App
    #[post("")]
    pub async fn post_app() -> impl Responder {
        HttpResponse::Ok().body("POST App")
    }

    //App
    #[get("")]
    pub async fn app() -> impl Responder {
        HttpResponse::Ok().body("GET App")
    }
}
