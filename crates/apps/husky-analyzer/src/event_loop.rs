mod dispatch;
mod send_updates;

use crossbeam_channel::select;

use std::error::Error;

use crate::{
    server::{Server, TaskSet},
    utils::{init_log_file, log_aux},
};

use self::send_updates::send_updates;

pub fn event_loop(connection: lsp_server::Connection) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut server = Server::new(connection.sender);
    init_log_file();
    while let Some(event) = select! {
        recv(connection.receiver) -> msg =>
            msg.ok().map(Event::Lsp),
        recv(server.event_loop_comm.receiver) -> task =>
            Some(Event::Task(task.unwrap())),
    } {
        log_aux(format!("{:?} begin dispatch", event));
        let loop_start = std::time::Instant::now();
        let task = match event {
            Event::Lsp(msg) => dispatch::dispatch_lsp_msg(&mut server, msg, loop_start)?,
            Event::Task(task) => task,
        };
        log_aux("end dispatch");
        match task {
            TaskSet::Nothing => (),
            TaskSet::Shutdown => {
                log_aux("exit peacefully");
                return Ok(());
            } // exit peacefully
            TaskSet::SendUpdates => {
                log_aux("send updates");
                send_updates(&server.db, &server.client_comm);
                log_aux("finish send updates");
            }
            TaskSet::Respond(response) => {
                log_aux("respond");
                server.client_comm.respond(response)
            }
        }
        log_aux("end event loop step");
    }
    log_aux("out of loop");
    Err("lsp client exited without proper shutdown sequence".into())
}

#[derive(Debug)]
pub enum Event {
    Lsp(lsp_server::Message),
    Task(TaskSet),
}
