use actix_web::*;

// this function could be located in a different module
pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(web::scope("/ui").service(index::hello));
}

//Index
pub mod index {
    use crate::view::setup;

    use actix_web::*;
    #[get("/index/hello")]
    pub async fn hello() -> impl Responder {
        let mut context = tera::Context::new();
        context.insert("hello_msg_from_rust", "Hello from Rust server with HTMX");
        let page_content = setup::TEMPLATES
            .render("index/components/hello_button.html", &context)
            .expect("Couldn't render index page");

        HttpResponse::Ok().body(page_content)
    }
}
