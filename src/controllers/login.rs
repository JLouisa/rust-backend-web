use crate::db::sqlite::SqliteDB;
use crate::domain::datatypes::{CookieVariations, Settings, UserClientSignIn, UserServer};
use crate::modules::cookie::generate_cookie;
use crate::modules::password_hash::Password;
use crate::modules::token_pub::generete_public_token;
use crate::view;
use actix_web::*;

pub async fn verify_login(db: web::Data<SqliteDB>, login_info: UserClientSignIn) -> HttpResponse {
    let user: Result<Option<UserServer>, sqlx::Error> =
        db.get_one_user_username(login_info.username.as_str()).await;

    match user {
        Ok(content) => match content {
            Some(user) => {
                let server_password = Password::new(&user.hashed_password);

                match server_password.verify_password(&login_info.password.as_str()) {
                    Ok(true) => {
                        let token = generete_public_token(user);
                        let cookie_settings =
                            Settings::new(token.as_str(), login_info.remember.unwrap_or(false));
                        let cookie = generate_cookie(&CookieVariations::Auth, cookie_settings);
                        HttpResponse::SeeOther()
                            .append_header(("Location", "/endpoints"))
                            .cookie(cookie)
                            .finish()
                    }

                    Ok(false) => {
                        let mut context = tera::Context::new();

                        context.insert("login_msg", "Please login to continue");
                        context.insert("login_value_username", &login_info.username);
                        context.insert("login_value_password", &login_info.password);
                        context.insert("login_failed_msg", "Username or Password is incorrect");
                        // context.insert("login_remember", "true");
                        match view::setup::TEMPLATES.render("pages/login/login.html", &context) {
                            Ok(content) => return HttpResponse::Ok().body(content),
                            Err(err) => {
                                eprintln!("Error rendering index page: {}", err);
                                // Return 500 Internal Server Error
                                return HttpResponse::InternalServerError().finish();
                            }
                        }
                    }
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            }
            None => {
                let mut context = tera::Context::new();

                context.insert("login_msg", "Please login to continue");
                context.insert("login_value_username", &login_info.username);
                context.insert("login_value_password", &login_info.password);
                context.insert("login_failed_msg", "Username or Password is incorrect");
                context.insert("login_remember", "true");
                match view::setup::TEMPLATES.render("pages/login/login.html", &context) {
                    Ok(content) => return HttpResponse::Ok().body(content),
                    Err(err) => {
                        eprintln!("Error rendering index page: {}", err);
                        // Return 500 Internal Server Error
                        return HttpResponse::InternalServerError().finish();
                    }
                }
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
