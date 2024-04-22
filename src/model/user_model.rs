use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
}

// #[derive(Queryable, Selectable)]
// #[table_name = "posts"]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct Post {
//     pub id: i32,
//     pub title: String,
//     pub text: String,
//     pub published: bool,
//     pub created_at: chrono::NaiveDateTime,
// }

// #[derive(Queryable, Selectable)]
// #[table_name = "posts"]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct Comment {
//     pub id: i32,
//     pub user: String,
//     pub title: String,
//     pub text: String,
//     pub published: bool,
//     pub created_at: chrono::NaiveDateTime,
// }
