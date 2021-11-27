use serde::{Deserialize, Deserializer, Serialize};

use super::address::Address;
use super::payload::PayloadInput;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageInput {
    Message(Message),
    Messages(Vec<Message>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InitMessageInput {
    Init(Message),
    Inits(Vec<Message>),
}

impl From<MessageInput> for Vec<Message> {
    fn from(other: MessageInput) -> Self {
        match other {
            MessageInput::Message(m) => vec![m],
            MessageInput::Messages(v) => v,
        }
    }
}

impl From<InitMessageInput> for Vec<Message> {
    fn from(other: InitMessageInput) -> Self {
        match other {
            InitMessageInput::Init(m) => vec![m],
            InitMessageInput::Inits(v) => v,
        }
    }
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<Message>, D::Error> {
    MessageInput::deserialize(deserializer).map(|v| v.into())
}

pub fn deserialize_init<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<Message>, D::Error> {
    InitMessageInput::deserialize(deserializer).map(|v| v.into())
}

pub fn deserialize_option<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Vec<Message>>, D::Error> {
    Option::<MessageInput>::deserialize(deserializer).map(|v| v.map(|x| x.into()))
}

pub fn deserialize_init_option<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Vec<Message>>, D::Error> {
    Option::<InitMessageInput>::deserialize(deserializer).map(|v| v.map(|x| x.into()))
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Message {
    pub id: Option<u64>,
    pub source: Option<Address>,
    pub dest: Option<Address>,
    #[serde(flatten)]
    pub payload: Option<PayloadInput>,
    pub gas_limit: Option<u64>,
    pub value: Option<u128>,
    pub reply_to: Option<u64>,
    pub exit_code: Option<i32>,
}
