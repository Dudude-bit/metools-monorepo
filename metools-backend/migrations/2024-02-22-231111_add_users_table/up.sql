-- Your SQL goes here
CREATE TABLE users (
    id uuid not null primary key ,
    created_at timestamp with time zone not null default now()
)