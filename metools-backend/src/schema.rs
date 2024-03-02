// @generated automatically by Diesel CLI.

diesel::table! {
    rzd_tasks (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        #[sql_name = "type"]
        type_ -> Text,
        data -> Jsonb,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(rzd_tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(rzd_tasks, users,);
