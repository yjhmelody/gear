#![no_std]

extern crate alloc;
use gstd::{debug, exec, msg, prelude::*, ProgramId};
use circular_buffer::CircularBuffer;
use codec::{Decode, Encode};
use primitive_types::H256;
use scale_info::TypeInfo;

gstd::metadata! {
    title: "GEAR Workshop Channel Contract",
    handle:
        input: ChannelAction,
        output: ChannelOutput,
}

#[derive(Debug, Encode, TypeInfo, Clone)]
struct Message {
    text: String,
    timestamp: u32,
}

#[derive(Debug, Encode, TypeInfo)]
struct Meta {
  name: String,
  description: String,
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

#[derive(Debug, Encode, TypeInfo)]
enum ChannelOutput {
  Metadata(Meta),
  SingleMessage(Message),
  MessageList(Vec<Message>),
}

struct State {
  channel_name: String,
  channel_description: String,
  owner_id: Option<ProgramId>,
  subscribers: Vec<ProgramId>,
  messages: Option<CircularBuffer<Message>>,
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
    self.messages.as_mut().unwrap().push(message);
  }
}

static mut STATE: State = State {
  channel_name: String::new(),
  channel_description: String::new(),
  owner_id: None,
  subscribers: Vec::new(),
  messages: None,
};

const GAS_RESERVE: u64 = 100_000_000;

#[no_mangle]
pub unsafe extern "C" fn init() {
  STATE.channel_name = "Test".to_string();
  STATE.channel_description = "Test description".to_string();
  STATE.messages = Some(CircularBuffer::new(5));
  STATE.set_owner_id(msg::source());

  let bh: u32 = exec::block_height();

  let init_message = Message {
    text: format!("Channel {} was created", STATE.channel_name).to_string(),
    timestamp: bh,
  };

  STATE.add_message(init_message);
  STATE.add_subscriber(STATE.owner_id.unwrap());
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: ChannelAction = msg::load().expect("Unable to decode Channel Action");
    let bh: u32 = exec::block_height();

    let source: ProgramId = msg::source();

    debug!("Received action: {:?}", action);

    let success_msg = Message {
      text: "success".to_string(),
      timestamp: 0,
    };

    match action {
      ChannelAction::Meta => {
        let meta = Meta {
          name: STATE.channel_name.clone(),
          description: STATE.channel_description.clone(),
          owner_id: program_id_to_hex(STATE.owner_id.unwrap()),
        };

        debug!("Sending meta information: {:?}", meta);

        msg::reply(ChannelOutput::Metadata(meta), exec::gas_available() - GAS_RESERVE, 0); // how to send meta?
      }
      ChannelAction::ChannelFeed => {
        let message_vector: Vec<Message> = STATE.messages.clone().unwrap().collect();

        debug!("Sending channel feed: {:?}", message_vector);

        msg::reply(ChannelOutput::MessageList(message_vector), exec::gas_available() - GAS_RESERVE, 0);
      }
      ChannelAction::Subscribe => {
        STATE.add_subscriber(source);

        debug!("Added a new subscriber: {:?}", source);

        msg::reply(ChannelOutput::SingleMessage(success_msg), exec::gas_available() - GAS_RESERVE, 0);
      }
      ChannelAction::Unsubscribe => {
        STATE.remove_subscriber(source);

        debug!("Removed a subscriber: {:?}", source);

        msg::reply(ChannelOutput::SingleMessage(success_msg), exec::gas_available() - GAS_RESERVE, 0);
      }
      ChannelAction::Post(text) => {
        if source != STATE.owner_id.unwrap() {
          debug!("User not authorized to add a post: {:?}", source);
          return;
        }

        let message = Message {
          text: text,
          timestamp: bh,
        };

        // send out notification messages
        for subscriber_id in STATE.subscribers.iter() {
          debug!("Sending a notification to: {:?}", &subscriber_id);
          msg::send(*subscriber_id, ChannelOutput::SingleMessage(message.clone()), 0, 0);
        }

        STATE.add_message(message.clone());

        debug!("Added a post: {:?}", &message);

        msg::reply(ChannelOutput::SingleMessage(success_msg), exec::gas_available() - GAS_RESERVE, 0);
      }
    }

}
