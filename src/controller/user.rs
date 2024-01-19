pub mod index {
    use actix_web::*;

    pub fn get_homepage() -> impl Responder {
        HttpResponse::Ok().body("Homepage")
    }
    pub fn get_hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }
}

pub mod user {
    use actix_web::*;

    pub fn get_all() -> impl Responder {
        HttpResponse::Ok().body("GET All Users")
    }
    pub fn get_one(path: web::Path<(String,)>) -> HttpResponse {
        HttpResponse::Ok().body(format!("GET User detail: {}", path.into_inner().0))
    }

    pub fn post_one(path: web::Path<(String,)>) -> HttpResponse {
        HttpResponse::Ok().body(format!("POST User detail: {}", path.into_inner().0))
    }
}

pub mod json {
    use actix_web::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User {
        id: usize,
        name: String,
    }
    impl User {
        fn new(id: usize, name: String) -> Self {
            Self { id, name }
        }
    }

    pub fn json_get() -> impl Responder {
        let user = User::new(1, String::from("Eve"));

        // Serde serialized
        // let serialized_user = serde_json::to_string(&user).expect("Failed to serialize");
        // HttpResponse::Ok().body(serialized_user)

        // Actix shorthand
        HttpResponse::Ok().json(user)
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
    pub fn json_post(item: web::Json<User>) -> impl Responder {
        HttpResponse::Ok().json(item.0)
    }
}
