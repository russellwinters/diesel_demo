// @generated automatically by Diesel CLI.

diesel::table! {
    players (id) {
        id -> Int4,
        #[max_length = 255]
        full_name -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        is_active -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    players,
    posts,
);
