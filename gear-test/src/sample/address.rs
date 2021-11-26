use serde::{Deserialize, Serialize};
use primitive_types::H256;

pub type Keyword = String;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum Address {
    Bind(Keyword),
    ChainAddress(ChainAddress),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ChainAddress {
    Id(u64),
    Account(H256),
}
