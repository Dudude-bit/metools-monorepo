-- Your SQL goes here
CREATE TABLE rzd_tasks(
    id uuid not null primary key ,
    type text not null ,
    data jsonb not null
)