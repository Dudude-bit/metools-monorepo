-- Your SQL goes here
CREATE TABLE users (
    id uuid not null primary key ,
    created_at timestamp with time zone default now()
)