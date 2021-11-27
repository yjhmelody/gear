use serde::{Deserialize, Serialize};

use super::address::Address;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Allocation {
    pub actor: Address,
    pub filter: Option<AllocationFilter>,
    pub kind: AllocationExpectationKind,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AllocationFilter {
    Static,
    Dynamic,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AllocationExpectationKind {
    PageCount(u64),
    ExactPages(Vec<u32>),
    ContainsPages(Vec<u32>),
}
