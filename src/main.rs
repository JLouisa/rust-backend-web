use actix_web::{get, middleware::Logger, web, App, Error, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use lib::{
    db::{self, sqlite::SqliteDB},
    models::schema::create_schema,
    routes::{app_routes, root_routes, ui_routes, users_routes},
    utils,
};
use serde::Serialize;

use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};

#[macro_use]
extern crate diesel_migrations;

#[derive(Serialize)]
pub struct Response {
    status: String,
    message: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(Response {
        status: "ok".to_string(),
        message: "Server is running".to_string(),
    })
}

async fn not_found_error() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotFound().json(Response {
        status: "error".to_string(),
        message: "Not Found".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    // Load environment variables from .env file
    dotenv().expect(".env file not found");
    env_logger::init();
    let port: u16 = utils::constants::PORT.clone();
    let address: String = utils::constants::ADDRESS.clone();
    let db_sqlite_url: String = utils::constants::DATABASE_SQLITE_URL.clone();

    // Setup Database Connection for Diesel
    let db_connection = db::database::Database::new();
    let app_data_pg = web::Data::new(db_connection);

    // Setup Database Connection for SQLX
    if !sqlx::Sqlite::database_exists(&db_sqlite_url)
        .await
        .expect("Something wrong 1")
    {
        let _ = sqlx::Sqlite::create_database(&db_sqlite_url).await;

        match create_schema(&db_sqlite_url).await {
            Ok(_) => println!("Database created Sucessfully"),
            Err(e) => panic!("{}", e),
        }
    }

    let database_sqlx = SqliteDB::new(&db_sqlite_url).await;
    let app_data_sqlx = web::Data::new(database_sqlx);

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(app_data_sqlx.clone())
            .wrap(Logger::default())
            .configure(app_routes::app_config)
            .configure(ui_routes::ui_config)
            .configure(users_routes::users_config)
            .configure(root_routes::root_config)
            .service(health)
            .service(root_routes::root::index)
        // .default_service(web::route().to(not_found_error()))
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
