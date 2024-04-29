use crate::db::database;
use crate::domain::user_domain;
use crate::schema::users::dsl::*;
use crate::{
    db::db_setup::establish_connection,
    models::{self, user_model::User},
};

use actix_web::*;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::{delete, insert_into, update};
use serde::{Deserialize, Serialize};

pub mod index {
    use super::*;

    pub fn get_homepage() -> impl Responder {
        HttpResponse::Ok().body("Homepage")
    }
    pub fn get_hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }
}

pub mod user {
    use self::database::Database;

    use super::*;

    pub fn get_all_users(db: web::Data<Database>) -> Option<user_domain::AllUserClient> {
        let the_users: Result<Vec<User>, DieselError> = db.get_all_users();

        match the_users {
            Ok(content) => {
                let all_users_client: Vec<user_domain::UserClient> = content
                    .iter()
                    .map(|user| user_domain::User::client(user))
                    .collect();

                let all_users = user_domain::AllUserClient {
                    users: all_users_client,
                };

                Some(all_users)
            }
            Err(err) => {
                eprintln!("Error loading users: {:?}", err);
                None
            }
        }
    }

    pub fn get_one_user(path: String, db: web::Data<Database>) -> HttpResponse {
        let user = db.get_one_user(path);
        match user {
            Some(user) => HttpResponse::Ok().json(user),
            None => HttpResponse::NotFound().finish(),
        }
    }

    pub fn post_one_user(user: models::user_model::User) -> HttpResponse {
        let connection = &mut establish_connection();

        let posted_user = insert_into(users).values(&user).execute(connection);

        match posted_user {
            Ok(content) => {
                println!("User posted: {:?}", content);
                let user = user_domain::User::client(&user);
                HttpResponse::Ok().json(user)
            }
            Err(e) => HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
        }
    }

    pub fn put_one_user(user: user_domain::UserClient) -> HttpResponse {
        // let connection = &mut establish_connection();
        let connection: &mut PgConnection = &mut establish_connection();

        let posted_user = update(users.find(&user.id))
            .set(username.eq(&user.username))
            .returning(User::as_returning())
            .get_result(connection);

        match posted_user {
            Ok(content) => {
                println!("User updated: {:?}", content);
                HttpResponse::Ok().json(content)
            }
            Err(e) => HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
        }
    }

    pub fn delete_one_user(user_id: web::Path<(String,)>) -> HttpResponse {
        let connection = &mut establish_connection();

        let posted_user = delete(users.find(&user_id.into_inner().0))
            .returning(User::as_returning())
            .get_result(connection);

        match posted_user {
            Ok(content) => {
                println!("User deleted: {:?}", content);
                let user = user_domain::User::client(&content);
                HttpResponse::Ok().json(user)
            }
            Err(e) => HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
        }
    }
}

pub mod json {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User2 {
        id2: usize,
        name: String,
    }

    impl User2 {
        fn new(id2: usize, name: String) -> Self {
            Self { id2, name }
        }

        pub fn json_get() -> impl Responder {
            let user = User2::new(1, String::from("Eve"));

            // Serde serialized
            // let serialized_user = serde_json::to_string(&user).expect("Failed to serialize");
            // HttpResponse::Ok().body(serialized_user)

            // Actix shorthand
            HttpResponse::Ok().json(user)
        }
    }

    // pub fn json_post(req_body: String) -> impl Responder {
    //     println!("It came in");
    //     let user_serialized = req_body;

    //     // Serde deserialized
    //     let user_deserialized: Result<User, _> = serde_json::from_str(&user_serialized);

    //     match user_deserialized {
    //         Ok(u) => HttpResponse::Ok().body(format!("ID: {:?}, and name: {:?}", u.id, u.name)),
    //         Err(e) => {
    //             HttpResponse::BadRequest().body(format!("Something went wrong. Error: {:?}", e))
    //         }
    //     }
    // }

    // Actix shorthand
    pub fn json_post(item: web::Json<User2>) -> impl Responder {
        HttpResponse::Ok().json(item.0)
    }
}
