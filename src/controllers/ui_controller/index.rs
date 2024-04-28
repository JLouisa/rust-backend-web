use crate::view::setup;
use crate::{controllers::user, db};
use actix_web::*;
use diesel::PgConnection;

pub mod index_ui_controller {
    use super::*;

    pub fn hello() -> impl Responder {
        let mut context = tera::Context::new();
        context.insert("hello_msg_from_rust", "Hello from Rust server with HTMX");
        let page_content = setup::TEMPLATES
            .render("index/components/hello_button.html", &context)
            .expect("Couldn't render index page");
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
            .render("index/components/ping_pong.html", &context)
            .expect("Couldn't render index page");
        HttpResponse::Ok().body(page_content)
    }

    pub fn post_one() -> HttpResponse {
        HttpResponse::Ok().body(format!("POST User detail: {}", "New User"))
    }

    pub fn show_all_user_list(connection: &mut PgConnection) -> HttpResponse {
        let all_users = user::user::get_all_users(connection);

        let mut context = tera::Context::new();

        match all_users {
            Some(content) => {
                context.insert("all_users", &content);
            }
            None => {
                let no_content = "No Users found";
                context.insert("all_users", &no_content);
            }
        }

        let page_content = setup::TEMPLATES
            .render("index/components/show_all_users.html", &context)
            .expect("Couldn't render index page");

        return HttpResponse::Ok().body(page_content);
    }

    pub fn deleted_user(user_id: String, connection: &mut PgConnection) -> HttpResponse {
        let deleted_user = db::database::users_db::delete_one_user(user_id, connection);

        let mut context = tera::Context::new();

        match deleted_user {
            Ok(content) => {
                context.insert("deleted_user", &content);
                let page_content = setup::TEMPLATES
                    .render("index/components/deleted_user.html", &context)
                    .expect("Couldn't render index page");

                return HttpResponse::Ok().body(page_content);
            }
            Err(err) => {
                return HttpResponse::BadRequest().body(format!("Error deleting user: {}", err))
            }
        }
    }
}
