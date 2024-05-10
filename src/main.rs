use crate::utils::constants::SHOP_CONFIGS;
use actix_web::{
    get, middleware::Logger, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use env_logger::Env;
use lib::{
    db::sqlite::SqliteDB,
    domain::shops::Shop,
    models::schema::create_schema,
    modules::{
        middleware,
        middleware_domain::AddShopDomain, // middleware_domain::ShopLoader
        middleware_msg::AddMsg,
        redis::RedisDB,
        stripe::stripe_webhooks::handle_webhook,
    },
    routes::{app_routes, root_routes, ui_routes, users_routes},
    utils,
    utils::constants::Config,
};
use serde::Serialize;
use sqlx::migrate::MigrateDatabase;

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

#[post("/stripe_webhooks")]
async fn webhook_handler(req: HttpRequest, payload: web::Bytes) -> HttpResponse {
    handle_webhook(req, payload).unwrap();
    HttpResponse::Ok().finish()
}

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
    // Logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load Configuration
    let config = Config::load_configuration();

    // Setup Database Connection for Diesel
    // let db_connection = db::database::Database::new();
    // let app_data_pg = web::Data::new(db_connection);

    // Setup Database Connection for SQLX
    if !sqlx::Sqlite::database_exists(&config.sqlx_database_url)
        .await
        .expect("Something wrong 1")
    {
        if create_schema(&config.sqlx_database_url).await.is_ok() {
            log::info!("Database created Sucessfully");
        } else {
            log::warn!("Failed to create database schema");
            panic!();
        }
    }
    let database_sqlx = SqliteDB::new(&config.sqlx_database_url).await;
    let app_data_sqlx = web::Data::new(database_sqlx);
    log::info!(
        "Sqlx Database pool created Sucessfully at {}",
        &&config.sqlx_database_url
    );

    // Setup Redis Connection
    let redis_db = match RedisDB::new(&config.redis_url) {
        Ok(db) => db,
        Err(e) => {
            log::warn!(
                "Failed to connect to Redis. Application cannot start without Redis: {}",
                e
            );
            panic!("Failed to connect to Redis");
        }
    };
    let app_data_redis = web::Data::new(redis_db);
    log::info!("Redis connection Sucessfull at {}", &&config.redis_url);

    load_shop_configs(&config.sqlx_database_url)
        .await
        .expect("Failed to load shop configurations");

    {
        // Should go in Redis - need to connect user to shop
        let mut shop_cfg = SHOP_CONFIGS.lock().unwrap();
        shop_cfg.insert(
            "localhost:3000".to_string(),
            Shop {
                name: "Localhost Shop".to_string(),
                product_type: "Localhost Product".to_string(),
            },
        );
        shop_cfg.insert(
            "honeydragons.com".to_string(),
            Shop {
                name: "honeydragons Shop".to_string(),
                product_type: "Fitness Products".to_string(),
            },
        );
    }

    // Log the server start
    log::info!("Starting HTTP server at http://localhost:{}", &config.port);

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(app_data_sqlx.clone())
            .app_data(app_data_redis.clone())
            .wrap(Logger::default())
            .wrap(AddMsg::enabled()) // Test middleware
            .wrap(AddShopDomain::enabled())
            .wrap(middleware::CheckLogin::disabled())
            .service(health)
            .service(webhook_handler)
            .configure(app_routes::app_config)
            .configure(ui_routes::ui_config)
            .configure(users_routes::users_config)
            .configure(root_routes::root_config)
            .service(root_routes::root::index_page)
    })
    .bind((config.address, config.port))?
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
