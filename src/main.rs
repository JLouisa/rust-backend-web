use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use lib::{
    routes::{app_routes, root_routes, ui_routes, users_routes},
    utils, view,
};

// use libsql::Builder;

// #[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    // Setup Database
    // let db = Builder::new_local("local.db").build().await.unwrap();
    // let conn_db = db.connect().unwrap();

    // Load environment variables from .env file
    dotenv().expect(".env file not found");
    env_logger::init();
    let port: u16 = utils::constants::PORT.clone();
    let address: String = utils::constants::ADDRESS.clone();

    // Index
    #[get("/")]
    pub async fn index() -> impl Responder {
        let mut context = tera::Context::new();
        context.insert("msg_from_rust", "Msg from Rust server");
        context.insert("ping_pong", "ping");

        match view::setup::TEMPLATES.render("index/index.html", &context) {
            Ok(content) => return HttpResponse::Ok().body(content),
            Err(err) => {
                eprintln!("Error rendering index page: {}", err);
                return HttpResponse::InternalServerError().finish(); // Return 500 Internal Server Error
            }
        };
    }

    // Start the server
    HttpServer::new(|| {
        App::new()
            .service(index)
            .wrap(Logger::default())
            .configure(ui_routes::app_config)
            .configure(app_routes::app_config)
            .configure(users_routes::users_config)
            .configure(root_routes::root_config)
    })
    .bind((address, port))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_hello_route() {
        // Arrange
        let mut app = test::init_service(App::new().service(root_routes::root::hello)).await;

        // Act
        let req = test::TestRequest::get().uri("/hello").to_request();
        let resp = test::call_service(&mut app, req).await;

        // Assert
        print!("{:?}", resp);
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello world!");
    }

    #[actix_rt::test]
    async fn test_users_route() {
        // Arrange
        let mut app =
            test::init_service(App::new().service(users_routes::user::get_all_user)).await;

        // Act
        let req = test::TestRequest::get().uri("users").to_request();
        let resp = test::call_service(&mut app, req).await;

        // Assert
        print!("{:?}", resp);
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "GET All Users");
    }
}
