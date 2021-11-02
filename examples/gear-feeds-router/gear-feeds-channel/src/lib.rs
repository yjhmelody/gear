#![no_std]

extern crate alloc;
use gstd::{exec, msg, prelude::*, ProgramId};
use ringbuffer::{AllocRingBuffer, RingBufferExt, RingBufferWrite};
use codec::{Decode, Encode};
use primitive_types::H256;
use scale_info::TypeInfo;

gstd::metadata! {
    title: "GEAR Workshop Channel Contract",
    handle:
        input: ChannelAction,
        output: String
}

#[derive(Debug, Encode, TypeInfo, Clone)]
struct Message {
    channel_id: H256,
    text: String,
    timestamp: u32,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
struct Channel {
  id: H256,
  name: String,
  owner_id: H256,
}

#[derive(Debug, Decode, TypeInfo)]
enum ChannelAction {
  Meta,
  ChannelFeed,
  Subscribe,
  Unsubscribe,
  Post(String),
}

struct State {
  channel_name: String,
  owner_id: Option<ProgramId>,
  subscribers: Vec<ProgramId>,
  messages: AllocRingBuffer<Message>,
}

fn program_id_to_hex(program_id: ProgramId) -> H256 {
  let ProgramId(bytes) = program_id;
  return H256::from(bytes);
}

impl State {
  fn set_owner_id(&mut self, user_id: ProgramId) {
    self.owner_id = Some(user_id);
  }

  fn add_subscriber(&mut self, subscriber_id: ProgramId) {
    self.subscribers.push(subscriber_id);
  }

  fn remove_subscriber(&mut self, subscriber_id: ProgramId) {
    let index = self.subscribers.iter().position(|x| *x == subscriber_id).expect("Subscriber doesn't exist.");
    self.subscribers.remove(index);
  }

  fn add_message(&mut self, message: Message) {
    self.messages.push(message);
  }
}

static mut STATE: State = State {
  channel_name: "Test".to_string(),
  owner_id: None,
  subscribers: Vec::new(),
  messages: AllocRingBuffer::with_capacity(5),
};

const GAS_RESERVE: u64 = 10_000_000;

#[no_mangle]
pub unsafe extern "C" fn init() {
  STATE.set_owner_id(msg::source());
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: ChannelAction = msg::load().expect("Unable to decode Channel Action");
    let bh: u32 = exec::block_height();

    let source: ProgramId = msg::source();

    match action {
      ChannelAction::Meta => {
        let meta = Channel {
          id: ,// how to get program id of this?
          name: STATE.channel_name,
          owner_id: program_id_to_hex(STATE.owner_id.unwrap()),
        };

        msg::reply(meta, exec::gas_available() - GAS_RESERVE, 0); // how to send meta?
      }
      ChannelAction::ChannelFeed => {
        msg::reply(STATE.messages.to_vec(), exec::gas_available() - GAS_RESERVE, 0);
      }
      ChannelAction::Subscribe => {
        STATE.add_subscriber(source);

        msg::reply("success", exec::gas_available() - GAS_RESERVE, 0);
      }
      ChannelAction::Unsubscribe => {
        STATE.remove_subscriber(source);

        msg::reply("success", exec::gas_available() - GAS_RESERVE, 0);
      }
      ChannelAction::Post(text) => {
        if source != STATE.owner_id.unwrap() {
          msg::reply("unauthorized", 0, 0);
          return;
        }

        let message = Message {
          channel_id: , // program id of this contract
          text: text,
          timestamp: bh,
        };

        // send out notification messages
        for subscriber_id in STATE.subscribers.iter() {
          msg::send(*subscriber_id, message, 0, 0);
        }

        STATE.add_message(message);

        msg::reply("success", exec::gas_available() - GAS_RESERVE, 0);
      }
    }

}
