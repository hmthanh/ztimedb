// @generated automatically by Diesel CLI.

diesel::table! {
    bandwidth (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    todos (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bandwidth,
    todos,
);
