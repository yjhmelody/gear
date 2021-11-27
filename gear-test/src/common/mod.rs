use gear_core::{
    message::{Message, MessageId, Payload},
    program::ProgramId,
};
use std::collections::BTreeMap;

pub type Result<T> = std::result::Result<T, String>;

use crate::sample::{
    self,
    address::Keyword,
    allocation::{AllocationExpectationKind, AllocationFilter},
};

pub struct Test {
    pub title: String,
    pub purpose: String,
    pub description: String,
    pub notebook: BTreeMap<Keyword, ProgramId>,
    pub programs: BTreeMap<ProgramId, WasmProgram>,
    pub fixtures: Vec<Fixture>,
}

pub struct WasmProgram {
    pub path: String,
    pub code: Vec<u8>,
    pub init: Option<Message>,
}

impl WasmProgram {
    pub fn new(path: String, init: Option<Message>) -> Result<Self> {
        let code = get_program_code(&path)?;
        Ok(Self { path, code, init })
    }
}

use sample::actor::Actor;

pub fn get_program_code(binary: &sample::actor::BinaryPath) -> Result<Vec<u8>> {
    std::fs::read(&binary).map_err(|_| format!("Unable to load program's code: {}", binary))
}

pub fn create_test(test: sample::test::Test) -> Result<Test> {
    let mut notebook: BTreeMap<Keyword, ProgramId> = BTreeMap::new();
    let mut messages: BTreeMap<u64, MessageId> = BTreeMap::new();
    let mut programs: BTreeMap<ProgramId, WasmProgram> = BTreeMap::new();
    let mut nonce = 0;

    for actor in test.actors {
        let (keyword, address) = actor.get_bind();
        let address: ProgramId = address.into();

        if let Some(v) = keyword {
            notebook.insert(v, address.clone());
        }

        if let Actor::Program(program) = actor {
            let message = match program.message {
                Some(message) => Some(create_message(
                    message,
                    &mut nonce,
                    &notebook,
                    &mut messages,
                )?),
                _ => None,
            };
            let wasm = WasmProgram::new(program.binary, message)?;
            programs.insert(address, wasm);
        }
    }

    let mut fixtures = Vec::new();

    let count = fixtures.len();

    for i in 0..count {
        let default_name = if count == 1 {
            String::from("main")
        } else {
            format!("#{}", i)
        };

        fixtures.push(create_fixture(
            default_name,
            notebook,
            programs,
            test.fixtures[i],
        )?);
    }

    Ok(Test {
        title: test.title,
        purpose: test.purpose.unwrap_or("Undefined".into()),
        description: test.description.unwrap_or("Undefined".into()),
        notebook,
        programs,
        fixtures,
    })
}

pub fn create_steps(
    steps: Vec<sample::step::Step>,
    nonce: &mut u64,
    notebook: &BTreeMap<Keyword, ProgramId>,
    msgs: &mut BTreeMap<u64, MessageId>,
) -> Result<Vec<Step>> {
    let mut current = 0;
    let mut found_final = false;
    let mut v = vec![];

    for step in steps {
        if found_final {
            return Err(format!());
        }
        if let Some(num) = step.number {
            if num < current {
                return Err(format!());
            }
            current = num
        } else {
            found_final = true;
        }

        let number = if found_final {
            StepNumber::Final
        } else {
            StepNumber::Ordered(current)
        };

        let immortal = step.immortal.unwrap_or(false);
        let messages = transform_messages(step.messages, nonce, notebook, msgs)?;
        let memory = transform(step.memory, &create_memory)?;
        let allocations = transform(step.allocations, &create_allocation)?;
        let waitlist = transform_messages(step.messages, nonce, notebook, msgs)?;
        let query = transform(step.query, &create_query)?;
        let mailbox = transform_messages(step.messages, nonce, notebook, msgs)?;

        v.push(Step {
            number,
            immortal,
            messages,
            memory,
            allocations,
            waitlist,
            query,
            mailbox,
        });
    }

    Ok(v)
}

pub fn transform<T, R>(
    input: Option<Vec<T>>,
    f: &dyn Fn(T) -> Result<R>,
) -> Result<Option<Vec<R>>> {
    Ok(match input {
        Some(dataset) => {
            let mut v = vec![];
            for data in dataset {
                v.push(f(data)?);
            }
            Some(v)
        }
        _ => None,
    })
}

pub fn transform_messages(
    input: Option<Vec<sample::message::Message>>,
    nonce: &mut u64,
    notebook: &BTreeMap<Keyword, ProgramId>,
    messages: &mut BTreeMap<u64, MessageId>,
) -> Result<Option<Vec<Message>>> {
    Ok(match input {
        Some(dataset) => {
            let mut v = vec![];
            for data in dataset {
                v.push(create_message(data, nonce, notebook, messages)?);
            }
            Some(v)
        }
        _ => None,
    })
}

const SUPERUSER: u64 = 0;

pub fn create_message(
    message: sample::message::Message,
    nonce: &mut u64,
    notebook: &BTreeMap<Keyword, ProgramId>,
    messages: &mut BTreeMap<u64, MessageId>,
) -> Result<Message> {
    let id: MessageId = (*nonce).into();
    *nonce += 1;

    if let Some(identifier) = message.id {
        messages.insert(identifier, id);
    }

    let source: ProgramId = match message.source {
        Some(sample::address::Address::Bind(keyword)) => {
            if let Some(id) = notebook.get(&keyword) {
                *id
            } else {
                return Err(format!(""));
            }
        }
        Some(sample::address::Address::ChainAddress(address)) => address.into(),
        None => SUPERUSER.into(),
    };

    let dest: ProgramId = match message.dest {
        Some(sample::address::Address::Bind(keyword)) => {
            if let Some(id) = notebook.get(&keyword) {
                *id
            } else {
                return Err(format!(""));
            }
        }
        Some(sample::address::Address::ChainAddress(address)) => address.into(),
        None => return Err(format!("")),
    };

    let payload = if let Some(payload) = message.payload {
        create_payload(payload)?
    } else {
        Vec::new().into()
    };

    let gas_limit = message.gas_limit.unwrap_or(u64::MAX);

    let value = message.value.unwrap_or_default();

    let reply = if let Some(id) = message.reply_to {
        if let Some(reply_to) = messages.get(&id) {
            Some((*reply_to, message.exit_code.unwrap_or(0)))
        } else {
            return Err(format!(""));
        }
    } else {
        if message.exit_code.is_some() {
            return Err(format!(""));
        }
        None
    };

    Ok(Message {
        id,
        source,
        dest,
        payload,
        gas_limit,
        value,
        reply,
    })
}

pub fn create_memory(_memory: sample::memory::Memory) -> Result<Memory> {
    Err(format!(""))
}

pub fn create_allocation(_allocation: sample::allocation::Allocation) -> Result<Allocation> {
    Err(format!(""))
}

pub fn create_payload(_payload: sample::payload::PayloadInput) -> Result<Payload> {
    Err(format!(""))
}

pub fn create_query(_query: sample::query::Query) -> Result<Query> {
    Err(format!(""))
}

pub struct Fixture {
    pub name: String,
    pub messages: BTreeMap<u64, MessageId>,
    pub notebook: BTreeMap<Keyword, ProgramId>,
    pub programs: BTreeMap<ProgramId, WasmProgram>,
    pub steps: Vec<Step>,
}

pub fn create_fixture(
    default_name: String,
    notebook: BTreeMap<Keyword, ProgramId>,
    programs: BTreeMap<ProgramId, WasmProgram>,
    fixture: sample::fixture::Fixture,
) -> Result<Fixture> {
    let name = fixture.name.unwrap_or(default_name);
    let mut nonce = programs.keys().len() as u64;
    let mut messages: BTreeMap<u64, MessageId> = BTreeMap::new();

    if let Some(inits) = transform_messages(fixture.inits, &mut nonce, &notebook, &mut messages)? {
        for init in inits {
            if let Some(entry) = programs.get_mut(&init.dest) {
                entry.init = Some(init);
            } else {
                return Err(format!(""));
            }
        }
    }

    if let Some(program) = programs.values().find(|v| v.init.is_none()) {
        return Err(format!(""));
    };

    let mut messages = BTreeMap::new();
    let steps = create_steps(fixture.steps, &mut nonce, &notebook, &mut messages)?;

    Ok(Fixture {
        name,
        notebook,
        programs,
        steps,
        messages,
    })
}

pub struct Step {
    pub number: StepNumber,
    pub immortal: bool,
    pub messages: Option<Vec<Message>>,
    pub memory: Option<Vec<Memory>>,
    pub allocations: Option<Vec<Allocation>>,
    pub waitlist: Option<Vec<Message>>,
    pub query: Option<Vec<Query>>,
    pub mailbox: Option<Vec<Message>>,
}

pub enum StepNumber {
    Final,
    Ordered(u8),
}

pub struct Memory {
    pub program_id: ProgramId,
    pub address: usize,
    pub bytes: Vec<u8>,
}

pub struct Allocation {
    pub program_id: ProgramId,
    pub filter: Option<AllocationFilter>,
    pub kind: AllocationExpectationKind,
}

pub struct Query {
    pub program_id: ProgramId,
    pub payload: Payload,
}

#[test]
fn name() {
    let yaml = r#"
    title: Test title

    purpose: Test purpose

    description: Test description

    actor:
      bind: pinger
      address:
        id: 1
      binary: target/wasm32-unknown-unknown/release/demo_async_multisig.wasm
      message:
        payload:
          hello: world

    fixtures:
    - name: Some fixture
      step:
        number: 10
    "#;

    let test: crate::sample::test::Test = serde_yaml::from_str(yaml).unwrap();

    println!("{:?}", test);
}
