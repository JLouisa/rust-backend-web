// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        hashed_password -> Text,
        created_at -> Timestamp,
        upgraded_at -> Timestamp,
    }
}
