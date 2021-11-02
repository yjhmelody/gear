#![no_std]
extern crate alloc;
use gstd_async::msg as msg_async;

use gstd::{exec, msg, prelude::*, ProgramId};

use alloc::collections::BTreeSet;
use codec::{Decode, Encode};
use primitive_types::H256;
use scale_info::TypeInfo;

gstd::metadata! {
  title: "GEAR Workshop Router Contract",
  handle:
      input: RouterAction,
      output: Vec<Channel>,
}

#[derive(Debug, Decode, Encode, TypeInfo, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Channel {
  id: H256,
  name: String,
  owner_id: H256,
}

#[derive(Debug, Decode, TypeInfo)]
enum RouterAction {
  Register(H256),
  Channels,
  Channel(H256),
}

#[derive(Debug, Encode, TypeInfo)]
enum ChannelAction {
  Meta,
  ChannelFeed,
  Subscribe,
  Unsubscribe,
  Post(String),
}

struct State {
  channels: BTreeSet<Channel>,
}

impl State {
  fn add_channel(&mut self, channel: &Channel) {
    STATE.channels.insert(*channel);
  }

  fn find_channel(&self, channel_id: H256) -> Channel {
    let channel = STATE.channels.into_iter().find(|&x| x.id == channel_id).unwrap();

    return channel;
  }

  fn get_vec_channels(&self) -> Vec<Channel> {
    return self.channels.iter().cloned().collect();
  }
}

static mut STATE: State = State { channels: vec![] };
const GAS_LIMIT: u64 = 50_000_000;
const GAS_RESERVE: u64 = 10_000_000;

#[no_mangle]
pub unsafe extern "C" fn init() {
}

#[gstd_async::main]
async fn main() {
    let action: RouterAction = msg::load().expect("Unable to decode Action");
    let bh: u32 = exec::block_height(); // block height

    let source = msg::source();

    match action {
      RouterAction::Register(hex) => {
        let channel_id = ProgramId(hex.to_fixed_bytes());

        let action = ChannelAction::Meta;
        // this will be changed when msg_async is fixed
        let reply = msg_async::send_and_wait_for_reply(channel_id, action.encode().as_ref(), GAS_LIMIT, 0).await;

        let meta = Channel::decode(&mut reply.as_ref())
          .expect("Unable to decode Channel meta");

        // if successfully decoded meta, store the channel info in the state
        STATE.add_channel(&meta);

        msg::reply(vec![meta], exec::gas_available() - GAS_RESERVE, 0);
      }
      RouterAction::Channel(hex) => {
        let channel_id = ProgramId(hex.to_fixed_bytes());

        let meta: Channel = STATE.find_channel(hex);

        msg::reply(vec![meta], exec::gas_available() - GAS_RESERVE, 0);
      }
      RouterAction::Channels => {
        let channels = STATE.get_vec_channels();

        msg::reply(channels, exec::gas_available() - GAS_RESERVE, 0);
      }
    }

}
