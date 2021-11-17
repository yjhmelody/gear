import { CreateType, DebugMode, GearApi, GearKeyring, ProgramDetails, GearMailbox, getWasmMetadata } from '@gear-js/api';
import testKeyring from '@polkadot/keyring/testing';
import { xxhashAsHex, blake2AsHex, randomAsHex } from '@polkadot/util-crypto';
import { Option } from '@polkadot/types';
import { Codec } from '@polkadot/types/types';
import YAML from 'yaml';
import * as fs from 'fs';
import { KeyringPair } from '@polkadot/keyring/types';
import { WsTestProvider } from './ws-test';

var metadata: any = {};

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function xxKey(module, key) {
  return xxhashAsHex(module, 128) + xxhashAsHex(key, 128).slice(2);
}

function findMessage(api, expMessage, snapshots, programs) {
  let found = -1;
  // console.log(programs);
  console.log('find msg');
  // console.log(expMessage.destination);

  // console.log(programs[expMessage.destination].toHuman());
  // console.log('snapshots len - ', snapshots.length);

  for (let index = 0; index < snapshots.length; index++) {
    const snapshot = snapshots[index];
    if (snapshot.messageQueue) {

      for (const message of snapshot.messageQueue) {

        // console.log('message.dest - ', message.dest);
        // console.log('expmessage.dest - ', programs[expMessage.destination]);

        if (message.dest.eq(programs[expMessage.destination])) {

          let payload: any;
          if (expMessage.payload) {
            if (expMessage.payload.kind === 'bytes') {

              payload = api.createType('Bytes', expMessage.payload.value);
            } else if (expMessage.payload.kind === 'i32') {
              payload = api.createType('i32', expMessage.payload.value);
            } else if (expMessage.payload.kind === 'i64') {
              payload = api.createType('i64', expMessage.payload.value);
            } else if (expMessage.payload.kind === 'f32') {
              payload = api.createType('f32', expMessage.payload.value);
            } else if (expMessage.payload.kind === 'f64') {
              payload = api.createType('f64', expMessage.payload.value);
            } else if (expMessage.payload.kind === 'utf-8') {
              if (expMessage.payload.value.search(/{([0-9]*)\}/) !== -1) {
                const res = expMessage.payload.value.match(/{([0-9]*)\}/);
                const id = Number(res[1]);
                if (programs[id] !== undefined) {
                  expMessage.payload.value = expMessage.payload.value.replace(res[0], programs[id].toString().slice(2));
                }
              }
              payload = api.createType('Bytes', expMessage.payload.value);
            } else if (expMessage.payload.kind === 'custom') {

              message.payload.value = JSON.stringify(expMessage.payload.value);
              if (message.payload.value.search(/{([0-9]*)\}/) !== -1) {
                const res = message.payload.value.match(/{([0-9]*)\}/);
                const id = Number(res[1]);
                if (programs[id] !== undefined) {
                  expMessage.payload.value = expMessage.payload.value.replace(res[0], programs[id].toString().slice(2));
                }
              }
              payload = CreateType.encode(metadata[expMessage.destination].handle_output, expMessage.payload.value, metadata[expMessage.destination]);
            }

            // console.log('exp payload - ', payload.toHuman());
            // console.log('msg payload - ', message.payload.toHuman());

            if (payload.eq(message.payload)) {

              found = index;
              break;
            }
          }
        } else {
          continue;
        }
      }
    }
  }
  return found;
}

async function resetStorage(api: GearApi, sudoPair: KeyringPair) {
  const keys = [];
  const txs = [];

  let hash = xxKey('Gear', 'MessageQueue');
  keys.push(hash);

  hash = xxKey('Gear', 'Mailbox');
  keys.push(hash);

  txs.push(api.tx.sudo.sudo(api.tx.system.killStorage(keys)));
  txs.push(api.tx.sudo.sudo(api.tx.system.killPrefix('g::', 1)));

  await api.tx.utility.batch(txs).signAndSend(sudoPair, { nonce: -1 });
  let head = (await api.rpc.state.getStorage('g::msg::head')) as Option<Codec>;
  while (head.isSome) {
    head = (await api.rpc.state.getStorage('g::msg::head')) as Option<Codec>;
  }
}

async function checkLog(api, exp, programs) {
  const errors = [];

  let mailbox = new GearMailbox(api);
  // use account id
  let messagesOpt = await mailbox.readMailbox('5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY');
  if (messagesOpt.isSome) {
    let messages = messagesOpt.unwrap();
    console.log(messages.toHuman());

    for (const log of exp.log) {
      let found = false;
      if ('payload' in log) {
        if (log.payload.kind === 'custom') {
          var encoded = CreateType.encode(metadata[1].handle_output, log.payload.value, metadata[1]);
          // console.log(metadata[log.payload]);
        }
        else {
          var encoded = CreateType.encode('Bytes', log.payload.value);
        }

        messages.forEach((message, _id) => {
          console.log(message.payload.toHuman(), encoded.toHuman());

          if (message.payload.toHex() == encoded.toHex()) {
            found = true;
            return;
          }
        });
        if (!found) {
          errors.push('Not Found');
        }
      }

    }
  } else {
    errors.push('Empty');
  }

  return errors;
}

async function checkMessages(api, exp, programs, snapshots) {
  console.log('checkMessages');
  // console.log(messageQueue.toHuman());
  // console.log(exp.messages);
  const errors = [];
  for (const snapshot of snapshots) {
    if (snapshot.messageQueue) {
      let messageQueue = snapshot.messageQueue;
      // if (exp.messages.length === 0 || exp.messages.length !== messageQueue.length) {
      //   errors.push(`Messages length doesn't match (expected: ${exp.messages.length}, recieved: ${messageQueue.length})`);
      //   return errors;
      // }
      let found = 0;
      for (let index = 0; index < exp.messages.length; index++) {
        const message = api.createType('Message', messageQueue[index]);
        const expMessage = exp.messages[index];
        found = findMessage(api, expMessage, snapshots.slice(found), programs);
        console.log(found);
        // console.log(payload, message.payload)
        // if (payload && !message.payload === payload.toU8a()) {
        //   errors.push(
        //     `Message payload doesn't match (expected: ${payload.toHuman()}, recieved: ${message.payload.toHuman()})`,
        //   );
        // }
        // if (!message.dest.eq(programs[expMessage.destination])) {
        //   errors.push(
        //     `Message destination doesn't match (expected: ${programs[
        //     expMessage.destination
        //     ]}, recieved: ${message.dest.toHuman()})`,
        //   );
        // }
        // if ('gas_limit' in expMessage) {
        //   if (!message.gas_limit.toNumber().eq(expMessage.gas_limit)) {
        //     errors.push(
        //       `Message gas_limit doesn't match (expected: ${expMessage.gas_limit
        //       }, recieved: ${message.gas_limit.toHuman()})`,
        //     );
        //   }
        // }
        // if ('value' in expMessage) {
        //   if (!message.value.toNumber().eq(expMessage.value)) {
        //     errors.push(
        //       `Message gas_limit doesn't match (expected: ${expMessage.value}, recieved: ${message.value.toHuman()})`,
        //     );
        //   }
        // }
        if (found === -1) {
          errors.push(
            `Message not found (expected: ${JSON.stringify(expMessage, null, 2)})`,
          );
          break;
        }
        // console.log('msg:', message.toHuman(), 'exp:', expMessage)
      }
    }

    if (errors.length > 0) {
      break;
    }
  }


  return errors;
}

async function checkMemory(api: GearApi, exp, programs) {
  const errors = [];

  for (const mem of exp.memory) {
    const gearProgramOpt = (await api.rpc.state.getStorage(
      `0x${Buffer.from('g::prog::').toString('hex')}${programs[mem.program_id].slice(2)}`,
    )) as Option<Codec>;
    const gearProgram = api.createType('Program', gearProgramOpt.unwrap()) as ProgramDetails;

    let at = parseInt(mem.at, 16);
    const bytes = Uint8Array.from(Buffer.from(mem.bytes.slice(2), 'hex'));
    const atPage = Math.floor(at / 65536);
    at -= atPage * 65536;

    let pages = [];

    for (let page of gearProgram.persistent_pages.keys()) {
      const buf = await api.rpc.state.getStorage(
        `0x${Buffer.from('g::prog::').toString('hex')}${programs[mem.program_id].slice(2)}::mem::${page.toHex()}`,
      );
      pages.push([page, buf]);
    }

    for (let [pageNumber, buf] of pages) {
      if (pageNumber == atPage) {
        for (let index = at; index < at + bytes.length; index++) {
          if (buf[index] !== bytes[index - at]) {
            errors.push("Memory doesn't match");
            break;
          }
        }
      }
    }
  }
  return errors;
}

async function processExpected(api, sudoPair, fixture, programs, snapshots) {
  const output = [];
  const errors = [];

  for (let expIdx = 0; expIdx < fixture.expected.length; expIdx++) {
    const exp = fixture.expected[expIdx];

    if (exp.step && exp.step < 2) {
      continue;
    }

    if ('messages' in exp) {

      const res = await checkMessages(api, exp, programs, snapshots);
      if (res.length === 0) {
        output.push('MSG: OK');
      } else {
        errors.push(`MSG ERR: ${res}`);
      }
    }


    if ('log' in exp) {
      const res = await checkLog(api, exp, programs);
      if (res.length === 0) {
        output.push('LOG: OK');
      } else {
        errors.push(`LOG ERR: ${res}`);
      }
    }

    // if ('memory' in exp) {
    //   const res = await checkMemory(api, exp, programs);
    //   if (res.length === 0) {
    //     output.push('MEMORY: OK');
    //   } else {
    //     errors.push(`MEMORY ERR: ${res}`);
    //   }
    // }
    if (errors.length > 0) {
      console.log(`Fixture ${fixture.title}`);
      for (const err of errors) {
        console.error(err);
      }
      process.exit(1);
    }
  }
  return output;
}

async function processFixture(api: GearApi, debugMode: DebugMode, sudoPair: KeyringPair, fixture: any, programs: any) {
  const txs = [];
  const snapshots = [];
  let res = [];

  // Send messages
  for (let index = 0; index < fixture.messages.length; index++) {
    const message = fixture.messages[index];
    let gas_limit = 100000000000;
    let value = 0;

    if (message.gas_limit) {
      gas_limit = message.gas_limit;
    }

    if (message.value) {
      value = message.value;
    }
    let payload: any;

    if (message.payload.kind === 'bytes') {
      payload = api.createType('Bytes', message.payload.value);
      console.log(api.createType('Bytes', message.payload.value).toHex(), message.payload.value);
    } else if (message.payload.kind === 'i32') {
      payload = api.createType('i32', message.payload.value).toU8a();
    } else if (message.payload.kind === 'i64') {
      payload = api.createType('i64', message.payload.value).toU8a();
    } else if (message.payload.kind === 'f32') {
      payload = api.createType('f32', message.payload.value).toU8a();
    } else if (message.payload.kind === 'f64') {
      payload = api.createType('f64', message.payload.value).toU8a();
    } else if (message.payload.kind === 'utf-8') {
      if (message.payload.value.search(/{([0-9]*)\}/) !== -1) {
        const res = message.payload.value.match(/{([0-9]*)\}/);
        const id = Number(res[1]);
        if (programs[id] !== undefined) {
          message.payload.value = message.payload.value.replace(res[0], programs[id].toString().slice(2));
        }
      }
      payload = api.createType('Bytes', message.payload.value);
      // } else if (message.payload.kind === 'custom') {
      //   if (message.payload.value.search(/{([0-9]*)\}/) !== -1) {
      //     const res = message.payload.value.match(/{([0-9]*)\}/);
      //     const id = Number(res[1]);
      //     if (programs[id] !== undefined) {
      //       message.payload.value = message.payload.value.replace(res[0], programs[id].toString().slice(2));
      //     }
      //   }
      //   payload = message.payload.value;
    } else if (message.payload.kind === 'custom') {
      message.payload.value = JSON.stringify(message.payload.value);
      if (message.payload.value.search(/{([0-9]*)\}/) !== -1) {
        const res = message.payload.value.match(/{([0-9]*)\}/);
        const id = Number(res[1]);
        if (programs[id] !== undefined) {
          message.payload.value = message.payload.value.replace(res[0], programs[id].toString().slice(2));
        }
      }
      payload = message.payload.value;
      // } else if (message.payload.kind === 'custom') {
      //   if (message.payload.value.search(/{([0-9]*)\}/) !== -1) {
      //     const res = message.payload.value.match(/{([0-9]*)\}/);
      //     const id = Number(res[1]);
      //     if (programs[id] !== undefined) {
      //       message.payload.value = message.payload.value.replace(res[0], programs[id].toString().slice(2));
      //     }
      //   }
      //   payload = message.payload.value;
    } else {
      payload = message.payload.value;
    }

    const meta = message.payload.kind === 'custom' ? metadata[message.destination] : { handle_input: 'Bytes' };

    console.log(message);

    txs.push(
      api.message.submit(
        {
          destination: programs[message.destination],
          payload: payload,
          gasLimit: gas_limit,
          value: 0,
        },
        meta,
      ),
    );
  }

  let sent = false;


  const unsub = await debugMode.snapshots(async ({ data }) => {
    // data.programs.forEach(({ id, static_pages, persistent_pages, code_hash, nonce }) => {
    //   console.log(`Program with id: ${id.toHuman()}`);
    // });
    data.messageQueue.forEach(({ id, source, dest, payload, gas_limit, value, reply }) => {
      console.log(`Message with id: ${id.toHuman()}`);
    });
    snapshots.push(data)
  });

  await api.tx.utility.batch(txs).signAndSend(sudoPair, { nonce: -1 });
  sent = true;
  await sleep(10000);
  unsub();

  return processExpected(api, sudoPair, fixture, programs, snapshots);
}

async function processTest(testData: any, api: GearApi, debugMode: DebugMode, sudoPair: KeyringPair) {
  const programs = {};
  const salt = {};
  const txs = [];
  for (const fixture of testData.fixtures) {
    const reset = await resetStorage(api, sudoPair);

    for (const program of testData.programs) {
      salt[program.id] = randomAsHex(20);
      let bytes = CreateType.encode('bytes', Array.from(fs.readFileSync(program.path)));
      let metaBytes = fs.readFileSync(program.path.replace('.wasm', '.meta.wasm'));
      programs[program.id] = api.program.generateProgramId(bytes, salt[program.id]);
      metadata[program.id] = await getWasmMetadata(metaBytes);
    }

    for (const program of testData.programs) {
      if (program.init_message) {
        let payload;
        const meta = program.init_message.kind === 'custom' ? metadata[program.id] : { init_input: 'Bytes' };
        if (program.init_message.kind === 'utf-8') {
          if (program.init_message.value.search(/\{([0-9]+)\}/g) !== -1) {
            const res = program.init_message.value.match(/\{([0-9]+)\}/g);
            for (const match of res) {
              const id = Number(match.slice(1, match.length - 1));
              if (programs[id] !== undefined) {
                program.init_message.value = program.init_message.value.replace(match, programs[id].toString().slice(2));
              }
            }
          }
          payload = api.createType('Bytes', program.init_message.value);
        } else if (program.init_message.kind === 'custom') {
          program.init_message.value = JSON.stringify(program.init_message.value);
          if (program.init_message.value.search(/\{([0-9]+)\}/g) !== -1) {
            const res = program.init_message.value.match(/\{([0-9]+)\}/g);
            for (const match of res) {
              const id = Number(match.slice(1, match.length - 1));
              if (programs[id] !== undefined) {
                program.init_message.value = program.init_message.value.replace(match, programs[id].toString().slice(2));
              }
            }
          }
          payload = program.init_message.value;
        }
        api.program.submit(
          {
            code: fs.readFileSync(program.path),
            salt: salt[program.id],
            initPayload: payload,
            gasLimit: 100000000000,
            value: 0,
          },
          meta,
        );
      } else {
        const meta = { init_input: 'Bytes' };
        api.program.submit(
          {
            code: fs.readFileSync(program.path),
            salt: salt[program.id],
            initPayload: [],
            gasLimit: 100000000000,
            value: 0,
          },
          meta,
        );
        // assert
      }
      txs.push(api.program.submitted);
    }

    await api.tx.utility.batch(txs).signAndSend(sudoPair, { nonce: -1 });

    const out = await processFixture(api, debugMode, sudoPair, fixture, programs);
    if (out.length > 0) {
      console.log(`Fixture ${fixture.title}: Ok`);
    }
  }
}

async function main() {
  const tests = [];

  // Load yaml files
  process.argv.slice(2).forEach((path) => {
    const fileContents = fs.readFileSync(path, 'utf8').toString();

    try {
      const data = YAML.parse(fileContents);
      tests.push(data);
    } catch (err) {
      console.error(err);
      process.exit(1);
    }
  });

  const totalFixtures = tests.reduce((tot, test) => tot + test.fixtures.length, 0);

  console.log('Total fixtures:', totalFixtures);

  // Create the API and wait until ready
  const api = await GearApi.create({ provider: new WsTestProvider('ws://127.0.0.1:9944') });
  const rootKeys = GearKeyring.fromSuri('//Alice', 'Alice default');

  console.log(rootKeys.address);

  const debugMode = new DebugMode(api);

  debugMode.enable();
  const isEnabled = await debugMode.signAndSend(rootKeys);
  console.log(isEnabled);

  // Retrieve the upgrade key from the chain state
  // const adminId = await api.query.sudo.key();

  // // Find the actual keypair in the keyring (if this is a changed value, the key
  // // needs to be added to the keyring before - this assumes we have defaults, i.e.
  // // Alice as the key - and this already exists on the test keyring)
  // const keyring = testKeyring.createTestKeyring();
  // const adminPair = keyring.getPair(adminId.toString());

  for (const test of tests) {
    if (test.skipRpcTest) continue;
    console.log('Test:', test.title);
    await processTest(test, api, debugMode, rootKeys);
  }
}

main()
  .catch((err) => {
    console.error(err);
    process.exit(1);
  })
  .finally(() => process.exit());
