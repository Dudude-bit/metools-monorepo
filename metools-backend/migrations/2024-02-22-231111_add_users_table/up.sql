-- Your SQL goes here
CREATE TABLE users (
    id uuid not null primary key ,
    username text not null unique ,
    email text not null unique ,
    password text not null ,
    created_at timestamp with time zone not null default now()
)