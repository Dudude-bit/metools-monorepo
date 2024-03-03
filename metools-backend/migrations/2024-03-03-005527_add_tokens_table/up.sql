-- Your SQL goes here
CREATE TABLE tokens (
    id uuid not null primary key ,
    created_at timestamp with time zone not null default now(),
    token uuid not null unique ,
    user_id uuid not null references users(id)
)