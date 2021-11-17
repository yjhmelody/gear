#![no_std]
#![feature(const_btree_new)]

use gstd::{debug, exec, msg, prelude::*, ActorId};

use codec::{Decode, Encode};
use primitive_types::H256;
use scale_info::TypeInfo;

gstd::metadata! {
  title: "GEAR Workshop Router Contract",
  handle:
      input: Register,
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

#[derive(Decode, TypeInfo)]
struct Meta {
    name: String,
    _description: String,
    _owner_id: H256,
}

#[gstd::async_main]
async fn main() {
    let register: Register = msg::load().expect("ROUTER: Unable to decode Register");

    debug!("ROUTER: Starting registering {:?}", register.0);

    let channel_id: ActorId = register.0.into();

    let ChannelOutput::Metadata(meta) = msg::send_and_wait_for_reply(
        channel_id,
        ChannelAction::Meta,
        exec::gas_available() - 100_000_000,
        0,
    )
    .await
    .expect("ROUTER: Error processing async message");

    msg::reply((), 0, 0);

    debug!(
        "ROUTER: Successfully added channel\nOwner: {:?}\nName: {:?}",
        register.0, meta.name
    );
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    debug!("Router Contract initialized successfully!");
}
