use crate::db::sqlite::SqliteDB;
use crate::domain::datatypes::{UserClientIn, UserServer};
use crate::modules::password_hash::Password;
use actix_web::*;

pub async fn verify_login(db: web::Data<SqliteDB>, login_info: UserClientIn) -> HttpResponse {
    let user: Result<Option<UserServer>, sqlx::Error> =
        db.get_one_user_username(login_info.username.as_str()).await;

    match user {
        Ok(content) => match content {
            Some(user) => {
                let server_password = Password::new(&user.hashed_password);

                match server_password.verify_password(&login_info.password.as_str()) {
                    Ok(verified) => {
                        println!("{:?}", verified);
                        if verified {
                            HttpResponse::SeeOther()
                                .header("Location", "/endpoints")
                                .finish()
                        } else {
                            HttpResponse::Ok().body("Wrong Password or Username")
                        }
                    }
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            }
            None => HttpResponse::Ok().body("Wrong Password or Username 2"),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
