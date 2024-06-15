use diesel::{r2d2, PgConnection};

pub mod rzd;
pub mod users;
pub mod verify_tokens;

pub type DBPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
