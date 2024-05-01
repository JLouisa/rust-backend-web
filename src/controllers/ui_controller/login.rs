use crate::view::setup;
use actix_web::*;

pub mod login_ui_controller {
    use super::*;

    pub fn login_page() -> HttpResponse {
        let mut context = tera::Context::new();

        context.insert("login_msg", "Please login to continue");
        let page_content = setup::TEMPLATES
            .render("index/components/deleted_user.html", &context)
            .expect("Couldn't render index page");

        return HttpResponse::Ok().body(page_content);
    }
}
