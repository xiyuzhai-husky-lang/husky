#![allow(warnings, unused)]

use std::error::Error;

use lsp_server::{Connection, Message, Request, RequestId, Response};
use lsp_types::{
    request::GotoDefinition, GotoDefinitionResponse, InitializeParams, ServerCapabilities,
};

use error_utils::Result;
use husky_analyzer::{event_loop, init_connection, utils::from_json};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        todo!()
    }
}

fn try_main() -> Result<()> {
    #[cfg(windows)]
    prioritize_this_thread();

    let (connection, io_threads) = Connection::stdio();

    init_connection(&connection)?;

    // ad hoc
    husky_profile_utils::init();

    event_loop(connection)?;

    io_threads.join()?;

    todo!();
}

#[cfg(windows)]
fn prioritize_this_thread() {
    // husky-lang-server team say:
    // Windows scheduler implements priority boosts: if thread waits for an
    // event (like a condvar), and event fires, priority of the thread is
    // temporary bumped. This optimization backfires in our case: each time the
    // `main_loop` schedules a task to run on a threadpool, the worker threads
    // gets a higher priority, and (on a machine with fewer cores) displaces the
    // main loop! We work-around this by marking the main loop as a
    // higher-priority thread.
    //
    // https://docs.microsoft.com/en-us/windows/win32/procthread/scheduling-priorities
    // https://docs.microsoft.com/en-us/windows/win32/procthread/priority-boosts
    // https://github.com/husky-lang-server/husky-lang-server/issues/2835
    unsafe {
        use winapi::um::processthreadsapi::*;
        let thread = GetCurrentThread();
        let thread_priority_above_normal = 1;
        SetThreadPriority(thread, thread_priority_above_normal);
    }
}
