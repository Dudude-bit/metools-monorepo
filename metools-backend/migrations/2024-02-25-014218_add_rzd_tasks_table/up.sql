-- Your SQL goes here
CREATE TABLE rzd_tasks(
    id uuid not null primary key ,
    created_at timestamp with time zone not null default now(),
    type text not null ,
    data jsonb not null,
    user_id uuid not null references users(id)
)