use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use lib::{
    db::sqlite::SqliteDB,
    domain::shops::Shop,
    models::schema::create_schema,
    modules::{
        middleware,
        middleware_domain::AddShopDomain, // middleware_domain::ShopLoader
        middleware_msg::AddMsg,
        redis::RedisDB,
    },
    routes::{app_routes, root_routes, ui_routes, users_routes},
    utils,
};
use serde::Serialize;
use sqlx::migrate::MigrateDatabase;

use crate::utils::constants::SHOP_CONFIGS;

// #[macro_use]
// extern crate diesel_migrations;

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

// async fn not_found_error() -> HttpResponse {
//     HttpResponse::NotFound().json(Response {
//         status: "error".to_string(),
//         message: "Not Found".to_string(),
//     })
// }

async fn load_shop_configs(database_url: &str) -> Result<(), sqlx::Error> {
    let pool = SqliteDB::new(database_url).await;
    let mut configs = SHOP_CONFIGS.lock().unwrap();
    let shops = SqliteDB::get_all_shop_domains(&pool).await;

    match shops {
        Ok(shops) => {
            for shop in shops {
                configs.insert(
                    shop.domain.to_string(),
                    Shop {
                        name: shop.name,
                        product_type: shop.product_type,
                    },
                );
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to load shop configurations: {}", e);
            return Err(e);
        }
    }
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
    // let db_connection = db::database::Database::new();
    // let app_data_pg = web::Data::new(db_connection);

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

    // Setup Redis Connection
    let redis_url = crate::utils::constants::REDIS_URL.clone();
    let redis_db = match RedisDB::new(&redis_url) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to connect to Redis: {}", e);
            // Decide how to handle the error: return an error or panic
            // For critical applications where Redis is mandatory, you might want to panic
            panic!("Application cannot start without Redis: {}", e);
        }
    };

    let app_data_redis = web::Data::new(redis_db);

    load_shop_configs(&db_sqlite_url)
        .await
        .expect("Failed to load shop configurations");

    {
        // Should go in Redis - need to connect user to shop
        let mut configs = SHOP_CONFIGS.lock().unwrap();
        configs.insert(
            "localhost:3000".to_string(),
            Shop {
                name: "Localhost Shop".to_string(),
                product_type: "Localhost Product".to_string(),
            },
        );
        configs.insert(
            "honeydragons.com".to_string(),
            Shop {
                name: "honeydragons Shop".to_string(),
                product_type: "Fitness Products".to_string(),
            },
        );
    }

    // Log the server start
    log::info!("starting HTTP server at http://localhost:{}", &port);

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(app_data_sqlx.clone())
            .app_data(app_data_redis.clone())
            .wrap(Logger::default())
            .wrap(AddMsg::enabled())
            .wrap(AddShopDomain::enabled())
            .wrap(middleware::CheckLogin::enabled())
            .configure(app_routes::app_config)
            .configure(ui_routes::ui_config)
            .configure(users_routes::users_config)
            .configure(root_routes::root_config)
            .service(root_routes::root::index_page)
            .service(health)
        // .default_service(not_found_error())
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
}
