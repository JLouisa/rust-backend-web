use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use lib::{
    routes::{app_routes, root_routes, users_routes},
    utils,
};

// #[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    // Load environment variables from .env file
    dotenv().ok();
    env_logger::init();
    let port: u16 = utils::constants::PORT.clone();
    let address: String = utils::constants::ADDRESS.clone();

    // Start the server
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
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
