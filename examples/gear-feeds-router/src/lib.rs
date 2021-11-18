#![no_std]

use gstd::{debug, exec, msg, prelude::*};

use codec::{Decode, Encode};
use primitive_types::H256;
use scale_info::TypeInfo;

gstd::metadata! {
  title: "GEAR Workshop Router Contract",
  handle:
      input: Register,
      output: Channel,
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

#[derive(Encode)]
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
    let register: Register = msg::load().expect("ROUTER: Unable to decode Register");

    debug!("ROUTER: Starting registering {:?}", register.0);

    let ChannelOutput::Metadata(meta) = msg::send_and_wait_for_reply(
        register.0.into(),
        ChannelAction::Meta,
        exec::gas_available() - 100_000_000,
        0,
    )
    .await
    .expect("ROUTER: Error processing async message");

    msg::reply(Channel::new(register.0, meta.clone()), 0, 0);

    debug!(
        "ROUTER: Successfully added channel\nOwner: {:?}\nName: {:?}",
        register.0, meta.name
    );
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    debug!("Router Contract initialized successfully!");
}
