use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Deserialize, Clone)]
pub struct Record {
    pub id: Thing,
}
