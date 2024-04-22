use actix_web::*;

// this function could be located in a different module
pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/ui")
            .service(index::hello)
            .service(index::ping_pong),
    );
}

//Index
pub mod index {
    use crate::controllers::ui_controller::index;
    use actix_web::*;

    #[get("/index/hello")]
    pub async fn hello() -> impl Responder {
        let response = index::index_ui_controller::hello();

        return response;
    }

    #[get("/index/mirror/{ping_pong}")]
    pub async fn ping_pong(path: web::Path<String>) -> impl Responder {
        let ping_pong: String = path.into_inner();
        println!("ping_pong: {}", ping_pong);
        let response = index::index_ui_controller::ping_pong(ping_pong);

        return response;
    }
}
