use crate::domain::datatypes::UserClientSignIn;
use crate::modules::middleware_msg::Msg;
use crate::{controllers, view};
use actix_web::web::{self, ReqData};
use actix_web::*;

// this function could be located in a different module
pub fn root_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(root::index_page)
            .service(root::endpoints_page)
            .service(root::login_page)
            .service(root::login_post)
            .service(root::forget_page)
            .service(root::forgot_post)
            .service(root::logout)
            .service(root::register_page)
            .service(root::shop_handler)
            .service(root::msg)
            .service(root::post_register)
            .service(root::reset_page)
            .service(root::reset_post)
            .service(root::echo)
            .service(root::hello)
            .service(root::json_post),
    );
}

// Root Routes Handlers (Controller)
pub mod root {
    use crate::{
        db::sqlite::SqliteDB,
        domain::datatypes::{
            CookieVariations, UserClientForgot, UserClientRegister, UserPassWordReset,
        },
        modules::middleware_domain::Shop,
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
    async fn shop_handler(shop: Option<ReqData<Option<Shop>>>) -> HttpResponse {
        match shop {
            Some(shop) => match shop.into_inner() {
                Some(shop) => HttpResponse::Ok().body(format!(
                    "Welcome to {}, selling {}",
                    shop.name, shop.product_type
                )),
                None => HttpResponse::NotFound().body("No shops found."),
            },
            None => HttpResponse::NotFound().body("No shops found."),
        }
    }

    // wrap route in our middleware factory
    #[get("/msg")]
    async fn msg(msg: Option<ReqData<Msg>>) -> HttpResponse {
        if let Some(msg_data) = msg {
            let Msg(message) = msg_data.into_inner();
            HttpResponse::Ok().body(message)
        } else {
            HttpResponse::InternalServerError().body("No message found.")
        }
    }

    #[get("/login")]
    pub async fn login_page() -> HttpResponse {
        let mut context = tera::Context::new();

        context.insert("login_msg", "Please login to continue");
        context.insert("login_value_username", "");
        context.insert("login_value_password", "");
        context.insert("login_failed_msg", "");
        match view::setup::TEMPLATES.render("pages/login/login.html", &context) {
            Ok(content) => return HttpResponse::Ok().body(content),
            Err(err) => {
                eprintln!("Error rendering login page: {}", err);
                return HttpResponse::InternalServerError().finish(); // Return 500 Internal Server Error
            }
        }
    }

    // POST Login info with remember field optional
    #[post("/login")]
    pub async fn login_post(
        db: web::Data<SqliteDB>,
        login_info: web::Form<UserClientSignIn>,
    ) -> impl Responder {
        let user = login_info.into_inner();

        controllers::login::verify_login(db, user).await
    }

    // Logout
    #[get("/logout")]
    pub async fn logout() -> HttpResponse {
        let cookie = CookieVariations::Auth.remove_cookie();
        HttpResponse::SeeOther()
            .append_header(("Location", "/login"))
            .cookie(cookie)
            .finish()
    }

    // Forgot Password
    #[get("/forgot")]
    pub async fn forget_page() -> HttpResponse {
        let mut context = tera::Context::new();

        context.insert("forgot_msg", "Please fill in your username to continue");
        context.insert("forgot_value_username", "");
        context.insert("forgot_msg", "");
        match view::setup::TEMPLATES.render("pages/forgot/forgot.html", &context) {
            Ok(content) => return HttpResponse::Ok().body(content),
            Err(err) => {
                eprintln!("Error rendering forgot page: {}", err);
                return HttpResponse::InternalServerError().finish(); // Return 500 Internal Server Error
            }
        }
    }

    // POST Login info with remember field optional
    #[post("/forgot")]
    pub async fn forgot_post(
        db: web::Data<SqliteDB>,
        user_info: web::Form<UserClientForgot>,
    ) -> impl Responder {
        let username = user_info.into_inner();

        let user_db = db.get_one_user_username(&username.username).await;

        match user_db {
            Ok(user) => match user {
                Some(user) => {
                    let username = UserClientForgot {
                        username: user.username.to_string(),
                    };
                    let result = crate::modules::email::send_password_reset_email(&username).await;

                    match result {
                        Ok(_) => {
                            let mut context = tera::Context::new();

                            context
                                .insert("forgot_msg", "Please fill in your username to continue");
                            context.insert("forgot_value_username", "");
                            context.insert(
                                "forgot_msg",
                                "Email sent successfully. Please check your email.",
                            );
                            match view::setup::TEMPLATES
                                .render("pages/forgot/forgot.html", &context)
                            {
                                Ok(content) => return HttpResponse::Ok().body(content),
                                Err(err) => {
                                    eprintln!("Error rendering forgot page: {}", err);
                                    // Return 500 Internal Server Error
                                    return HttpResponse::InternalServerError().finish();
                                }
                            }
                        }
                        Err(_) => {
                            let mut context = tera::Context::new();

                            context
                                .insert("forgot_msg", "Please fill in your username to continue");
                            context.insert("forgot_value_username", user.username.as_str());
                            context.insert("forgot_msg", "Something went wrong. Please try again.");
                            match view::setup::TEMPLATES
                                .render("pages/forgot/forgot.html", &context)
                            {
                                Ok(content) => return HttpResponse::Ok().body(content),
                                Err(err) => {
                                    eprintln!("Error rendering forgot page: {}", err);
                                    // Return 500 Internal Server Error
                                    return HttpResponse::InternalServerError().finish();
                                }
                            }
                        }
                    }
                }
                None => HttpResponse::Ok().body("User not found"),
            },
            Err(_) => {
                HttpResponse::InternalServerError().body("Something went wrong. Please try again.")
            }
        }
    }

    #[get("/reset/{username}")]
    pub async fn reset_page(path: web::Path<String>) -> HttpResponse {
        let user = path.into_inner();

        let mut context = tera::Context::new();

        context.insert("reset_msg", "Reset Password");
        context.insert("username", user.as_str());
        context.insert("reset_value_password", "");
        context.insert("reset_value_confirm_password", "");
        context.insert("reset_msg", "");
        match view::setup::TEMPLATES.render("pages/reset/reset.html", &context) {
            Ok(content) => return HttpResponse::Ok().body(content),
            Err(err) => {
                eprintln!("Error rendering reset page: {}", err);
                return HttpResponse::InternalServerError().finish(); // Return 500 Internal Server Error
            }
        }
    }

    // POST Reset info with remember field optional
    #[post("/reset/{username}")]
    pub async fn reset_post(
        db: web::Data<SqliteDB>,
        path: web::Path<String>,
        info: web::Form<UserPassWordReset>,
    ) -> impl Responder {
        let username = path.into_inner();
        let pwds = info.into_inner();

        let new_info = UserPassWordReset::verify_password(&pwds, username.to_string());

        match new_info {
            Ok(user) => {
                let result = db.update_one_user_password(&user).await;

                match result {
                    Ok(user) => {
                        match user {
                            Some(_) => HttpResponse::SeeOther()
                                .append_header(("Location", "/login"))
                                .finish(),
                            None => {
                                let mut context = tera::Context::new();

                                context.insert("reset_msg", "Reset Password");
                                context.insert("username", username.as_str());
                                context.insert("reset_value_password", "");
                                context.insert("reset_value_confirm_password", "");
                                context.insert("reset_msg", "User not found");
                                match view::setup::TEMPLATES
                                    .render("pages/reset/reset.html", &context)
                                {
                                    Ok(content) => return HttpResponse::Ok().body(content),
                                    Err(err) => {
                                        eprintln!("Error rendering reset page: {}", err);
                                        // Return 500 Internal Server Error
                                        return HttpResponse::InternalServerError().finish();
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {
                        let mut context = tera::Context::new();

                        context.insert("reset_msg", "Reset Password");
                        context.insert("username", username.as_str());
                        context.insert("reset_value_password", "");
                        context.insert("reset_value_confirm_password", "");
                        context.insert("reset_msg", "Something went wrong. Please try again.");
                        match view::setup::TEMPLATES.render("pages/reset/reset.html", &context) {
                            Ok(content) => return HttpResponse::Ok().body(content),
                            Err(err) => {
                                eprintln!("Error rendering reset page: {}", err);
                                // Return 500 Internal Server Error
                                return HttpResponse::InternalServerError().finish();
                            }
                        }
                    }
                }
            }
            Err(_) => {
                let mut context = tera::Context::new();

                context.insert("reset_msg", "Reset Password");
                context.insert("username", username.as_str());
                context.insert("reset_value_password", "");
                context.insert("reset_value_confirm_password", "");
                context.insert("reset_msg", "Passwords do not match");
                match view::setup::TEMPLATES.render("pages/reset/reset.html", &context) {
                    Ok(content) => return HttpResponse::Ok().body(content),
                    Err(err) => {
                        eprintln!("Error rendering reset page: {}", err);
                        // Return 500 Internal Server Error
                        return HttpResponse::InternalServerError().finish();
                    }
                }
            }
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
