use crate::common::{Fixture, Test, WasmProgram};

use anyhow::Result;

use gear_backend_common::Environment;
use gear_backend_wasmtime::WasmtimeEnvironment;
use gear_core::{
    message::Message,
    program::{Program, ProgramId},
    storage::{InMemoryStorage, Storage, StorageCarrier},
};
use gear_core_runner::{
    Config, ExtMessage, InitializeProgramInfo, RunNextResult, RunResult, Runner,
};
use gear_node_runner::{Ext, ExtStorage};
use std::collections::VecDeque;
use std::fmt::Write;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

// messages vecdeque
// pop message
// run
// append run result
// check the outcome if needed

pub fn init_program<SC: StorageCarrier, E: Environment<Ext>>(
    runner: &mut Runner<SC, E>,
    program_id: ProgramId,
    program: WasmProgram,
) -> Result<RunResult> {
    let message = program
        .init
        .expect("Checked in fixture generation. Can't fail");

    runner.init_program(InitializeProgramInfo {
        new_program_id: program_id,
        source_id: message.source(),
        code: program.code,
        message: ExtMessage {
            id: message.id(),
            payload: message.payload().into(),
            gas_limit: message.gas_limit(),
            value: message.value(),
        },
    })
}

pub fn run<SC: StorageCarrier, E: Environment<Ext>>(
    runner: &mut Runner<SC, E>,
    message: Message,
    block_height: u32,
) -> RunNextResult {
    runner.set_block_height(block_height);

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);

    runner.set_block_timestamp(timestamp as _);

    let mut run_result = runner.run_next(message);
    runner.process_wait_list(&mut run_result);

    run_result
}

pub fn check() {}

pub fn process_tests(_tests: Vec<Test>) {}

pub fn process_test(_test: Test) {}

use std::marker::{Sync, Send};

type WasmRunner<SC> = Runner<SC, gear_backend_wasmtime::WasmtimeEnvironment<Ext>>;

pub fn process_fixture<SC, F>(fixture: Fixture, storage_factory: F) -> Result<()>
where
    SC: StorageCarrier,
    F: Fn() -> Storage<SC::PS> + Sync + Send,
{
    let mut runner: WasmRunner<SC> = Runner::new(
        &Config::default(),
        storage_factory(),
        Default::default(),
        gear_backend_wasmtime::WasmtimeEnvironment::<Ext>::default(),
    );

    let mut messages: VecDeque<Message> = VecDeque::new();
    let mut log: VecDeque<Message> = VecDeque::new();

    for (id, program) in fixture.programs {
        let result = init_program(&mut runner, id, program)?;
                    // Messages that were generated during the run.
                // messages: Vec<OutgoingMessage>,
                    // Reply that was received during the run.
                // reply: Option<ReplyMessage>,
                    // Messages to be woken.
                // awakening: Vec<(MessageId, u64)>,
                    // Gas that was left.
                // gas_left: u64,
                    // Gas that was spent.
                // gas_spent: u64,
                    // Run outcome (trap/success/waiting).
                // outcome: ExecutionOutcome,
    }

    Ok(())
}
