use serde::{Deserialize, Deserializer, Serialize};

use super::allocation::Allocation;
use super::memory::Memory;
use super::message::Message;
use super::query::Query;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StepInput {
    Step(Step),
    Steps(Vec<Step>),
}

impl From<StepInput> for Vec<Step> {
    fn from(other: StepInput) -> Self {
        match other {
            StepInput::Step(s) => vec![s],
            StepInput::Steps(v) => v,
        }
    }
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<Step>, D::Error> {
    StepInput::deserialize(deserializer).map(|v| v.into())
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Step {
    pub number: Option<u8>,
    pub immortal: Option<bool>,
    #[serde(flatten)]
    #[serde(deserialize_with = "super::message::deserialize_option")]
    pub messages: Option<Vec<Message>>,
    pub memory: Option<Vec<Memory>>,
    pub allocations: Option<Vec<Allocation>>,
    pub waitlist: Option<Vec<Message>>,
    pub query: Option<Vec<Query>>,
    pub mailbox: Option<Vec<Message>>,
}
