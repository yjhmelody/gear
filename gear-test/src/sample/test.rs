use serde::{Serialize, Deserialize};

use super::actor::Actor;
use super::fixture::Fixture;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test {
    pub title: String,
    pub purpose: Option<String>,
    pub description: Option<String>,
    #[serde(flatten)]
    #[serde(deserialize_with = "super::actor::deserialize")]
    pub actors: Vec<Actor>,
    #[serde(flatten)]
    #[serde(deserialize_with = "super::fixture::deserialize")]
    pub fixtures: Vec<Fixture>,
}
