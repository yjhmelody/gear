use hex::FromHex;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_yaml::Value;

use super::address::Address;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Memory {
    pub program_id: Address,
    #[serde(rename = "at")]
    #[serde(deserialize_with = "de_address")]
    pub address: usize,
    #[serde(deserialize_with = "de_bytes")]
    pub bytes: Vec<u8>,
}

fn de_address<'de, D: Deserializer<'de>>(deserializer: D) -> Result<usize, D::Error> {
    if let Value::String(s) = Value::deserialize(deserializer)? {
        let v = usize::from_str_radix(s.trim_start_matches("0x"), 16).map_err(de::Error::custom)?;
        return Ok(v);
    }

    Err(de::Error::custom("wrong type"))
}

fn de_bytes<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    if let Value::String(s) = Value::deserialize(deserializer)? {
        let v = Vec::from_hex(s.trim_start_matches("0x")).map_err(de::Error::custom)?;
        return Ok(v);
    }

    Err(de::Error::custom("wrong type"))
}
