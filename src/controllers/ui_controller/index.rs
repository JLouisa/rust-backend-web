use self::db::diesel::Database;
use crate::db;
use crate::view::setup;
use actix_web::*;

pub mod index_ui_controller {
    use self::db::sqlite::SqliteDB;

    use super::*;

    pub fn hello() -> impl Responder {
        let mut context = tera::Context::new();
        context.insert("hello_msg_from_rust", "Hello from Rust server with HTMX");
        let page_content = setup::TEMPLATES
            .render("pages/endpoints/components/hello_button.html", &context)
            .expect("Couldn't render hello page");
        HttpResponse::Ok().body(page_content)
    }

    pub fn ping_pong(mirror: String) -> impl Responder {
        let mirror = match mirror.as_str() {
            "ping" => "pong",
            "pong" => "ping",
            _ => "ping",
        };

        let mut context = tera::Context::new();

        context.insert("ping_pong", mirror);
        let page_content = setup::TEMPLATES
            .render("pages/endpoints/components/ping_pong.html", &context)
            .expect("Couldn't render endpoints ping pong");
        HttpResponse::Ok().body(page_content)
    }

    pub fn post_one() -> HttpResponse {
        HttpResponse::Ok().body(format!("POST User detail: {}", "New User"))
    }

    pub async fn show_all_user_list(db: web::Data<SqliteDB>) -> HttpResponse {
        let all_users = db.get_all_users().await;

        let mut context = tera::Context::new();

        match all_users {
            Ok(content) => {
                context.insert("all_users", &content);
            }
            Err(_) => {
                let no_content = "No Users found";
                context.insert("all_users", &no_content);
            }
        }

        let page_content = setup::TEMPLATES
            .render("pages/endpoints/components/show_all_users.html", &context)
            .expect("Couldn't render show all user list page");

        return HttpResponse::Ok().body(page_content);
    }

    pub fn show_all_user_list_diesel(db: web::Data<Database>) -> HttpResponse {
        let all_users = db.get_all_users();

        let mut context = tera::Context::new();

        match all_users {
            Ok(content) => {
                context.insert("all_users", &content);
            }
            Err(_) => {
                let no_content = "No Users found";
                context.insert("all_users", &no_content);
            }
        }

        let page_content = setup::TEMPLATES
            .render("pages/endpoints/components/show_all_users.html", &context)
            .expect("Couldn't render show all user list page");

        return HttpResponse::Ok().body(page_content);
    }

    pub fn deleted_user(user_id: String, db: web::Data<Database>) -> HttpResponse {
        let deleted_user = db.delete_one_user(user_id);

        let mut context = tera::Context::new();

        match deleted_user {
            Ok(content) => {
                context.insert("deleted_user", &content);
                let page_content = setup::TEMPLATES
                    .render("pages/endpoints/components/deleted_user.html", &context)
                    .expect("Couldn't render deleted user page");

                return HttpResponse::Ok().body(page_content);
            }
            Err(err) => {
                return HttpResponse::BadRequest().body(format!("Error deleting user: {}", err))
            }
        }
    }

    pub async fn deleted_user_sqlite(user_id: String, db: web::Data<SqliteDB>) -> HttpResponse {
        match db.get_one_user(user_id.as_str()).await {
            Ok(the_user) => {
                let deleted_user = db.delete_one_user(user_id.as_str()).await;
                let mut context = tera::Context::new();
                match deleted_user {
                    Ok(_) => {
                        context.insert("deleted_user", &the_user);
                        let page_content = setup::TEMPLATES
                            .render("pages/endpoints/components/deleted_user.html", &context)
                            .expect("Couldn't render deleted user page");

                        return HttpResponse::Ok().body(page_content);
                    }
                    Err(err) => {
                        return HttpResponse::BadRequest()
                            .body(format!("Error deleting user: {}", err))
                    }
                }
            }
            Err(err) => {
                return HttpResponse::BadRequest().body(format!("Error deleting user: {}", err))
            }
        }
    }
}
