DEFINE TABLE users SCHEMAFULL;

DEFINE FIELD username ON users TYPE string;
DEFINE FIELD email ON users TYPE string;
DEFINE FIELD password ON users TYPE string;
DEFINE FIELD created_at ON users VALUE time::now() READONLY;
DEFINE FIELD is_verified ON users TYPE bool DEFAULT false;
DEFINE FIELD role ON users TYPE string DEFAULT 'user';

DEFINE INDEX users_username_index ON users COLUMNS username UNIQUE;
DEFINE INDEX users_email_index ON users COLUMNS email UNIQUE;
