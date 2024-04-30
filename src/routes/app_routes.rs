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
            .service(app::sqlite_create_one)
            .service(app::sqlite_update_one)
            .service(app::sqlite_delete_one),
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

    // GET
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

    // GET
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

    // POST
    #[post("/sqlite/create")]
    pub async fn sqlite_create_one(
        db: web::Data<SqliteDB>,
        user: web::Json<UserClientIn>,
    ) -> impl Responder {
        let user = UserServer::process_for_server(user.into_inner());

        match db.create_one_user(&user).await {
            Ok(user) => {
                return HttpResponse::Ok().json(user.process_for_client());
            }
            Err(err) => {
                eprintln!("Error getting user: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };
    }

    // PUT
    #[put("/sqlite/users")]
    pub async fn sqlite_update_one(
        db: web::Data<SqliteDB>,
        user: web::Json<UserServer>,
    ) -> impl Responder {
        let user = user.into_inner();

        match db.update_one_user(&user).await {
            Ok(content) => {
                return HttpResponse::Ok().json(content);
            }
            Err(err) => {
                eprintln!("Error updating user2: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };
    }

    // DELETE
    #[delete("/sqlite/users/{id}")]
    pub async fn sqlite_delete_one(
        db: web::Data<SqliteDB>,
        path: web::Path<String>,
    ) -> impl Responder {
        let user_id: String = path.into_inner();

        match db.delete_one_user(&user_id).await {
            Ok(msg) => return HttpResponse::Ok().json(msg),
            Err(err) => {
                eprintln!("Error deleting user: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };
    }
}
