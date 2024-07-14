use std::str::FromStr;

use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use surrealdb::sql::Thing;

pub fn decode_from_base64_string(s: String) -> String {
    String::from_utf8(BASE64_URL_SAFE_NO_PAD.decode(s).unwrap()).unwrap()
}

pub fn encode_to_base64_string(s: String) -> String {
    BASE64_URL_SAFE_NO_PAD.encode(s)
}

pub fn decode_from_base64_to_thing(s: String) -> Thing {
    Thing::from_str(decode_from_base64_string(s).as_str()).unwrap()
}

pub fn encode_thing_to_base64_string(t: Thing) -> String {
    encode_to_base64_string(t.to_string())
}
