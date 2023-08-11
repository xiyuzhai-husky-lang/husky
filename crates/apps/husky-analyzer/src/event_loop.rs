mod dispatch;
mod send_updates;

use crossbeam_channel::select;

use std::error::Error;

use crate::server::{Server, TaskSet};

use self::send_updates::send_updates;

pub fn event_loop(connection: lsp_server::Connection) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut server = Server::new(connection.sender);
    while let Some(event) = select! {
        recv(connection.receiver) -> msg =>
            msg.ok().map(Event::Lsp),
        recv(server.event_loop_comm.receiver) -> task =>
            Some(Event::Task(task.unwrap())),
    } {
        let loop_start = std::time::Instant::now();
        let task = match event {
            Event::Lsp(msg) => dispatch::dispatch_lsp_msg(&mut server, msg, loop_start)?,
            Event::Task(task) => task,
        };
        match task {
            TaskSet::Nothing => (),
            TaskSet::Shutdown => {
                return Ok(());
            } // exit peacefully
            TaskSet::SendUpdates => send_updates(&server.db, &server.client_comm),
            TaskSet::Respond(response) => server.client_comm.respond(response),
        }
    }
    Err("lsp client exited without proper shutdown sequence".into())
}

#[derive(Debug)]
pub enum Event {
    Lsp(lsp_server::Message),
    Task(TaskSet),
}
