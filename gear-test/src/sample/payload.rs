use serde::{Serialize, Deserialize};
use serde_yaml::Value;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PayloadInput {
    Payload(Value),
    PayloadBytes(String)
}
