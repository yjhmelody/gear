use gear_core::program::ProgramId;
use primitive_types::H256;
use serde::{Deserialize, Serialize};

pub type Keyword = String;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum Address {
    Bind(Keyword),
    ChainAddress(ChainAddress),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ChainAddress {
    Id(u64),
    Account(H256),
}

impl From<ChainAddress> for ProgramId {
    fn from(other: ChainAddress) -> Self {
        match other {
            ChainAddress::Id(id) => ProgramId::from(id),
            ChainAddress::Account(h256) => ProgramId::from(h256.to_fixed_bytes()),
        }
    }
}
