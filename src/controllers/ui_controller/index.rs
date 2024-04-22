use crate::view::setup;
use actix_web::*;

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
}
