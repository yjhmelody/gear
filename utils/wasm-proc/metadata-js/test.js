// We use the assert standard library to make assertions
const assert = require('assert');
const fs = require('fs');
const path = require('path');
const wasmMetadata = require('.');

let wasmBytes = fs.readFileSync(
  path.join('../../../target/wasm32-unknown-unknown/release/non_fungible_token.meta.wasm'),
);

wasmMetadata.getWasmMetadata(wasmBytes).then((metadata) => {
  console.log(metadata);
  //   metadata.registry = JSON.stringify(reg.createType('PortableRegistry', metadata.registry).toHuman());

  //   assert.deepStrictEqual(metadata, {
  //     init_input: 'MessageInitIn',
  //     init_output: 'MessageInitOut',
  //     async_init_input: 'MessageInitAsyncIn',
  //     async_init_output: 'MessageInitAsyncOut',
  //     handle_input: 'MessageIn',
  //     handle_output: 'MessageOut',
  //     async_handle_input: 'MessageHandleAsyncIn',
  //     async_handle_output: 'MessageHandleAsyncOut',
  //     state_input: 'Option<Id>',
  //     state_output: 'Vec<Wallet>',
  //     registry:
  //     title: 'Example program with metadata',
  //   });
});

// async
// wasmBytes = fs.readFileSync(
//   path.join(__dirname, '../../../target/wasm32-unknown-unknown/release/demo_async.meta.wasm'),
// );

// wasmMetadata.getWasmMetadata(wasmBytes).then((metadata) => {
//   const reg = new TypeRegistry();
//   metadata.registry = JSON.stringify(reg.createType('PortableRegistry', metadata.registry).toHuman());

//   assert.deepStrictEqual(metadata, {
//     init_input: 'Vec<u8>',
//     init_output: 'Vec<u8>',
//     async_init_input: '',
//     async_init_output: '',
//     handle_input: 'Vec<u8>',
//     handle_output: 'Vec<u8>',
//     async_handle_input: '',
//     async_handle_output: '',
//     state_input: '',
//     state_output: '',
//     registry:
//       '{"types":[{"id":"0","type":{"path":[],"params":[],"def":{"Sequence":{"type":"1"}},"docs":[]}},{"id":"1","type":{"path":[],"params":[],"def":{"Primitive":"U8"},"docs":[]}}]}',
//     title: 'demo async',
//   });
// });
