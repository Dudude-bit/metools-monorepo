use serde::{Deserialize, Deserializer, Serialize};
use surrealdb::sql::Thing;

use super::string::{decode_from_base64_to_thing, encode_thing_to_base64_string};

#[derive(Clone)]
pub struct Base64EncodedThing(pub Thing);

impl<'de> Deserialize<'de> for Base64EncodedThing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(Base64EncodedThing(decode_from_base64_to_thing(s)))
    }
}

impl Serialize for Base64EncodedThing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(encode_thing_to_base64_string(self.0.clone()).as_str())
    }
}
