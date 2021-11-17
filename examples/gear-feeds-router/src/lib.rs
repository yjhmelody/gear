#![no_std]
#![feature(const_btree_new)]

use gstd::{debug, exec, msg, prelude::*, ProgramId};
use gstd_async::msg as msg_async;

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

#[gstd_async::main]
async fn main() {
    let register: Register = msg::load().expect("ROUTER: Unable to decode Register");

    debug!("ROUTER: Starting registering {:?}", register.0);

    let channel_id = ProgramId(register.0.to_fixed_bytes());
    let channel_action = ChannelAction::Meta.encode();

    let reply = msg_async::send_and_wait_for_reply(
        channel_id,
        channel_action.as_ref(),
        exec::gas_available() - 100_000_000,
        0,
    )
    .await
    .expect("ROUTER: Error processing async message");

    let ChannelOutput::Metadata(meta) =
        ChannelOutput::decode(&mut reply.as_ref()).expect("Unable to decode Meta of the channel");

    debug!(
        "ROUTER: Successfully added channel\nOwner: {:?}\nName: {:?}",
        register.0, meta.name
    );

    msg::reply((), 0, 0);
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    debug!("Router Contract initialized successfully!");
}
