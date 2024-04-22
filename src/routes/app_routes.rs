use actix_web::*;

// this function could be located in a different module
pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(web::scope("/app").service(app::post_app).service(app::app));
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
