use serde::{Deserialize, Serialize, Deserializer};

use super::address::{ChainAddress, Keyword};
use super::message::Message;

pub type BinaryPath = String;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ActorInput {
    Actor(Actor),
    Actors(Vec<Actor>),
}

impl From<ActorInput> for Vec<Actor> {
    fn from(other: ActorInput) -> Self {
        match other {
            ActorInput::Actor(a) => vec![a],
            ActorInput::Actors(v) => v,
        }
    }
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<Actor>, D::Error> {
    ActorInput::deserialize(deserializer).map(|v| v.into())
}

pub fn deserialize_option<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Vec<Actor>>, D::Error> {
    ActorInput::deserialize(deserializer).map(|v| Some(v.into()))
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Actor {
    Program(Program),
    User(User),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Program {
    pub bind: Option<Keyword>,
    pub address: ChainAddress,
    pub binary: BinaryPath,
    pub message: Option<Message>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub bind: Option<Keyword>,
    pub address: ChainAddress,
}
