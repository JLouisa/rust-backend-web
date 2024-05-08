use crate::utils::constants::SHOP_CONFIGS;
use crate::{controllers, view};
use actix_web::*;

// this function could be located in a different module
pub fn root_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(root::index_page)
            .service(root::endpoints_page)
            .service(root::register_page)
            .service(root::shop_handler)
            .service(root::shop_handler2)
            .service(root::post_register)
            .service(root::echo)
            .service(root::hello)
            .service(root::json_post),
    );
}

// Root Routes Handlers (Controller)
pub mod root {
    use crate::{
        db::sqlite::SqliteDB, domain::datatypes::UserClientRegister,
        modules::middleware_domain::get_shop_domain,
    };

    use super::*;

    // Index
    #[get("/")]
    pub async fn index_page() -> impl Responder {
        let mut context = tera::Context::new();
        context.insert("home_msg_from_rust", "Msg from Rust server");
        context.insert("ping_pong", "ping");

        match view::setup::TEMPLATES.render("pages/index/index.html", &context) {
            Ok(content) => return HttpResponse::Ok().body(content),
            Err(err) => {
                eprintln!("Error rendering index page: {}", err);
                return HttpResponse::InternalServerError().finish(); // Return 500 Internal Server Error
            }
        };
    }

    #[get("/endpoints")]
    pub async fn endpoints_page() -> impl Responder {
        let mut context = tera::Context::new();
        context.insert("msg_from_rust", "Msg from Rust server");
        context.insert("ping_pong", "ping");

        match view::setup::TEMPLATES.render("pages/endpoints/endpoints.html", &context) {
            Ok(content) => return HttpResponse::Ok().body(content),
            Err(err) => {
                eprintln!("Error rendering index page: {}", err);
                return HttpResponse::InternalServerError().finish(); // Return 500 Internal Server Error
            }
        };
    }

    #[get("/register")]
    pub async fn register_page() -> HttpResponse {
        let mut context = tera::Context::new();

        context.insert("register_msg", "Please register to continue");
        context.insert("register_value_username", "");
        context.insert("register_value_password", "");
        context.insert("register_value_confirm_password", "");
        context.insert("register_failed_msg", "");
        match view::setup::TEMPLATES.render("pages/register/register.html", &context) {
            Ok(content) => return HttpResponse::Ok().body(content),
            Err(err) => {
                eprintln!("Error rendering index page: {}", err);
                return HttpResponse::InternalServerError().finish(); // Return 500 Internal Server Error
            }
        }
    }

    #[post("/register")]
    pub async fn post_register(
        info: web::Form<UserClientRegister>,
        db: web::Data<SqliteDB>,
    ) -> HttpResponse {
        let user_info = info.into_inner();
        let user = user_info.verify_password();
        let mut context = tera::Context::new();

        if user.is_ok() {
            let created_user = db.create_one_user(&user.unwrap()).await;

            if created_user.is_ok() {
                return HttpResponse::SeeOther()
                    .append_header(("Location", "/login"))
                    .finish();
            } else {
                context.insert(
                    "register_failed_msg",
                    "Something went wrong when creating you account",
                );
            }
        } else {
            context.insert("register_failed_msg", "Password is not the same");
        }
        context.insert("register_msg", "Please register to continue");
        context.insert("register_value_username", &user_info.username);
        context.insert("register_value_password", &user_info.password);
        context.insert(
            "register_value_confirm_password",
            &user_info.confirm_password,
        );
        match view::setup::TEMPLATES.render("pages/register/register.html", &context) {
            Ok(content) => return HttpResponse::Ok().body(content),
            Err(err) => {
                eprintln!("Error rendering index page: {}", err);
                return HttpResponse::InternalServerError().finish(); // Return 500 Internal Server Error
            }
        }
    }

    #[get("/shop")]
    async fn shop_handler(req: HttpRequest) -> HttpResponse {
        let shop = get_shop_domain(req).await;

        match shop.1 {
            Some(config) => HttpResponse::Ok().body(format!(
                "Welcome to {}, selling {} at domain {}",
                config.name, config.product_type, shop.0
            )),
            None => HttpResponse::NotFound().body(format!("Shop not found for domain {}", shop.0)),
        }
    }

    #[get("/shop2")]
    async fn shop_handler2(req: HttpRequest) -> HttpResponse {
        // Extract hostname from the request
        let host = req
            .headers()
            .get("host")
            .map(|v| v.to_str().unwrap_or_default())
            .unwrap_or_default()
            .to_string();

        // Log the hostname for debugging or tracking
        println!("Request received for host: {}", host);

        // Access the global configuration for shops
        let shop_configs = SHOP_CONFIGS.lock().unwrap(); // Using a mutex to guard global state
        let domain = host.split(':').next().unwrap_or_default().to_string();

        match shop_configs.get(&domain) {
            Some(config) => HttpResponse::Ok().body(format!(
                "Welcome to {}, selling {} at domain {}",
                config.name, config.product_type, domain
            )),
            None => HttpResponse::NotFound().body(format!("Shop not found for domain {}", domain)),
        }
    }

    //Hello
    #[get("/hello")]
    pub async fn hello() -> impl Responder {
        controllers::user::index::get_hello()
    }

    //Echo
    #[post("/echo")]
    pub async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }

    //POST JSON
    type TheUser = controllers::user::json::User2;
    #[post("/json")]
    pub async fn json_post(item: web::Json<TheUser>) -> impl Responder {
        controllers::user::json::json_post(item)
    }
}
