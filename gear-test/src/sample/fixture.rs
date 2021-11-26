use serde::{Serialize, Deserialize, Deserializer};

use super::message::Message;
use super::step::Step;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FixtureInput {
    Fixture(Fixture),
    Fixtures(Vec<Fixture>),
}

impl From<FixtureInput> for Vec<Fixture> {
    fn from(other: FixtureInput) -> Self {
        match other {
            FixtureInput::Fixture(a) => vec![a],
            FixtureInput::Fixtures(v) => v,
        }
    }
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<Fixture>, D::Error> {
    FixtureInput::deserialize(deserializer).map(|v| v.into())
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Fixture {
    pub name: Option<String>,
    #[serde(flatten)]
    #[serde(deserialize_with = "super::message::deserialize_init_option")]
    pub inits: Option<Vec<Message>>,
    #[serde(flatten)]
    #[serde(deserialize_with = "super::step::deserialize")]
    pub steps: Vec<Step>,
}
