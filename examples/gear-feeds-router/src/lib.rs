#![no_std]

use gstd::{debug, exec, msg, prelude::*};

use codec::{Decode, Encode};
use primitive_types::H256;
use scale_info::TypeInfo;

gstd::metadata! {
  title: "GEAR Workshop Router Contract",
  handle:
      input: RouterAction,
      output: Vec<Channel>,
}

#[derive(Debug, Decode, TypeInfo)]
enum RouterAction {
    Register(H256),
    Channels,
    Channel(H256),
}

#[derive(Decode, TypeInfo)]
struct Register(H256);

#[derive(Encode, TypeInfo)]
enum ChannelAction {
    Meta,
}

#[derive(Decode, TypeInfo)]
enum ChannelOutput {
    Metadata(Meta),
}

#[derive(Clone, Decode, TypeInfo)]
struct Meta {
    name: String,
    description: String,
    owner_id: H256,
}

#[derive(Encode, TypeInfo)]
struct Channel {
    id: H256,
    name: String,
    owner_id: H256,
    description: String,
}

impl Channel {
    fn new(id: H256, meta: Meta) -> Self {
        Self {
            id,
            name: meta.name,
            owner_id: meta.owner_id,
            description: meta.description,
        }
    }
}

#[gstd::async_main]
async fn main() {
    let register: RouterAction = msg::load().expect("ROUTER: Unable to decode RouterAction");

    match register {
        RouterAction::Register(hex) => {
            debug!("ROUTER: Starting registering {:?}", hex);

            let ChannelOutput::Metadata(meta) = msg::send_and_wait_for_reply(
                hex.into(),
                ChannelAction::Meta,
                exec::gas_available() - 100_000_000,
                0,
            )
            .await
            .expect("ROUTER: Error processing async message");

            msg::reply(vec![Channel::new(hex, meta.clone())], 0, 0);

            debug!(
                "ROUTER: Successfully added channel\nName: {:?}\nAddress: {:?}\nOwner: {:?}",
                meta.name, hex, meta.owner_id
            );
        }
        _ => debug!("Got another action"),
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    debug!("Router Contract initialized successfully!");
}
