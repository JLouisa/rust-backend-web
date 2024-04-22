// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        hashed_password -> Text,
        created_at -> Timestamp,
        upgraded_at -> Timestamp,
    }
}
