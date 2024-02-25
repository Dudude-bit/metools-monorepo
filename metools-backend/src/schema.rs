use diesel;

diesel::table! {
    use diesel::sql_types::*;
    users (id) {
        id -> Uuid,
        created_at -> Timestamptz
    }
}

diesel::table! {
    use diesel::sql_types::*;
    rzd_tasks (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        #[sql_name="type"]
        _type -> Text,
        data -> Jsonb,
        user_id -> Uuid
    }
}