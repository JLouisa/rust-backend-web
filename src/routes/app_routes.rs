use crate::db::sqlite::SqliteDB;
use crate::domain::datatypes::{UserClientIn, UserServer};
use actix_web::*;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

// this function could be located in a different module
pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/app")
            .service(app::app)
            .service(app::post_app)
            .service(app::sqlite_all_one)
            .service(app::sqlite_get_one)
            .service(app::sqlite_create_one),
    );
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    id: String,
    username: String,
    password: String,
    active: bool,
}

pub mod app {
    use super::*;

    //App
    #[get("")]
    pub async fn app() -> impl Responder {
        HttpResponse::Ok().body("GET App")
    }

    //App
    #[post("")]
    pub async fn post_app() -> impl Responder {
        HttpResponse::Ok().body("POST App")
    }

    // SqliteDB
    #[get("/sqlite/users")]
    pub async fn sqlite_all_one(db: web::Data<SqliteDB>) -> impl Responder {
        match db.get_all_user().await {
            Ok(users) => {
                return HttpResponse::Ok().json(users);
            }
            Err(err) => {
                eprintln!("Error getting user: {:?}", err);
                return HttpResponse::NotFound().json("Users not found");
            }
        };
    }

    // SqliteDB
    #[get("/sqlite/users/{id}")]
    pub async fn sqlite_get_one(
        db: web::Data<SqliteDB>,
        path: web::Path<String>,
    ) -> impl Responder {
        let user_id: String = path.into_inner();

        match db.get_one_user(&user_id).await {
            Ok(content) => {
                return HttpResponse::Ok().json(content.process_for_client());
            }
            Err(err) => {
                eprintln!("Error getting user: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };
    }

    // SqliteDB
    #[post("/sqlite/create")]
    pub async fn sqlite_create_one(
        db: web::Data<SqliteDB>,
        user: web::Json<UserClientIn>,
    ) -> impl Responder {
        let user = UserServer::process_for_server(user.into_inner());

        match db.create_one_user(&user).await {
            Ok(content) => {
                return HttpResponse::Ok().json(content.process_for_client());
            }
            Err(err) => {
                eprintln!("Error getting user: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };
    }
}
