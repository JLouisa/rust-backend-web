use crate::db::sqlite::SqliteDB;
use crate::domain::datatypes::{UserClientIn, UserServer};
use actix_web::*;

// this function could be located in a different module
pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/app")
            .service(sqlite::app)
            .service(sqlite::post_app)
            .service(sqlite::sqlite_all_one)
            .service(sqlite::sqlite_get_one)
            .service(sqlite::sqlite_create_one)
            .service(sqlite::sqlite_update_one)
            .service(sqlite::sqlite_transaction)
            .service(sqlite::sqlite_delete_one),
    );
}

pub mod sqlite {
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

    // TRANSACTION
    #[post("/sqlite/transaction")]
    pub async fn sqlite_transaction(
        db: web::Data<SqliteDB>,
        user: web::Json<UserClientIn>,
    ) -> impl Responder {
        let user = UserServer::process_for_server(user.into_inner());

        match db.transaction(&user).await {
            Ok(user) => {
                return HttpResponse::Ok().json(user.process_for_client());
            }
            Err(err) => {
                eprintln!("Something went Wrong with the Transaction: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };
    }
}
