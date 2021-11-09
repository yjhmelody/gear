#![no_std]
#![feature(const_btree_new)]

extern crate alloc;

use gstd_async::msg as msg_async;
use gstd::{debug, exec, msg, prelude::*, ProgramId};

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
  description: String,
  owner_id: H256,
}

#[derive(Debug, Decode, TypeInfo)]
struct Meta {
  name: String,
  description: String,
  owner_id: H256,
}

#[derive(Debug, Decode, TypeInfo)]
enum RouterAction {
  Register(H256),
  Channels,
  Channel(H256),
}

#[derive(Debug, Decode, TypeInfo)]
enum ChannelOutput {
  Metadata(Meta),
}

#[derive(Debug, Encode, TypeInfo)]
enum ChannelAction {
  Meta,
}

struct State {
  channels: BTreeSet<Channel>,
}

impl State {
  fn add_channel(&mut self, channel: Channel) -> bool {
    unsafe { STATE.channels.insert(channel) }
  }

  fn find_channel(&self, channel_id: H256) -> Channel {
    let channels = unsafe { STATE.channels.clone() };
    let channel = channels.into_iter().find(|x| x.id == channel_id).unwrap();

    return channel;
  }

  fn get_vec_channels(&self) -> Vec<Channel> {
    return self.channels.iter().cloned().collect();
  }
}

static mut STATE: State = State { channels: BTreeSet::new() };
const GAS_LIMIT: u64 = 500_000_000;
const GAS_RESERVE: u64 = 100_000_000;

#[no_mangle]
pub unsafe extern "C" fn init() {
}

#[gstd_async::main]
async fn main() {
    let action: RouterAction = msg::load().expect("Unable to decode Action");

    debug!("Received action: {:?}", action);

    match action {
      RouterAction::Register(hex) => {
        let channel_id = ProgramId(hex.to_fixed_bytes());

        let action = ChannelAction::Meta;
        // this will be changed when msg_async is fixed
        let reply = msg_async::send_and_wait_for_reply(channel_id, action.encode().as_ref(), GAS_LIMIT, 0).await;

        debug!("Recived reply {:?}", reply);

        let ChannelOutput::Metadata(meta) = ChannelOutput::decode(&mut reply.as_ref())
          .expect("Unable to decode Meta of the channel");
          
        debug!("Recived meta {:?}", meta);
          
        let channel = Channel {
          id: hex,
          name: meta.name,
          description: meta.description,
          owner_id: meta.owner_id,
        };

        // if successfully decoded meta, store the channel info in the state
        unsafe { STATE.add_channel(channel.clone()) };

        debug!("Added a new channel {:?}", hex);

        msg::reply(vec![channel], exec::gas_available() - GAS_RESERVE, 0);
      }
      RouterAction::Channel(hex) => {
        let meta: Channel = unsafe { STATE.find_channel(hex).clone() };

        debug!("Requested info about channel {:?}", hex);

        msg::reply(vec![meta], exec::gas_available() - GAS_RESERVE, 0);
      }
      RouterAction::Channels => {
        let channels = unsafe { STATE.get_vec_channels() };

        debug!("Sending a list of all channels: {:?}", channels);

        msg::reply(channels, exec::gas_available() - GAS_RESERVE, 0);
      }
    }

}
