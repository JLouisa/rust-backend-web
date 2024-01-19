use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use lib::routes::routing;
use std::env;

// #[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    let port: u16 = match env::var("PORT") {
        Ok(the_port) => the_port.parse().expect("PORT should be a number"),
        Err(_) => 8080,
    };

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/users")
                    .service(routing::user::get_all_user)
                    .service(routing::user::get_one_user),
            )
            .service(
                web::scope("/app")
                    .service(routing::app::app)
                    .service(routing::app::post_app),
            )
            .service(routing::root::index)
            .service(routing::root::echo)
            .service(routing::root::hello)
            .service(routing::root::json_get)
            .service(routing::root::json_post)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_index_route() {
        // Arrange
        let mut app = test::init_service(App::new().service(routing::root::index)).await;

        // Act
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        // Assert
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Homepage");
    }

    #[actix_rt::test]
    async fn test_hello_route() {
        // Arrange
        let mut app2 = test::init_service(App::new().service(routing::root::hello)).await;

        // Act
        let req = test::TestRequest::get().uri("/hello").to_request();
        let resp = test::call_service(&mut app2, req).await;

        // Assert
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello world!");
    }

    #[actix_rt::test]
    async fn test_users_route() {
        // Arrange
        let mut app = test::init_service(
            App::new().service(web::scope("/users").service(routing::user::get_all_user)),
        )
        .await;

        // Act
        let req = test::TestRequest::get().uri("/users").to_request();
        let resp = test::call_service(&mut app, req).await;

        // Assert
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "GET All Users");
    }
}
