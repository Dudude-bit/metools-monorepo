-- Your SQL goes here
CREATE TABLE verify_tokens (
    id uuid not null primary key ,
    created_at timestamp with time zone not null default now(),
    valid_until timestamp with time zone not null,
    token uuid not null unique ,
    user_id uuid not null references users(id)
)
