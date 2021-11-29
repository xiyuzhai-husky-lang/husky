mod event;
mod handle_event;

use std::error::Error;

use crossbeam_channel::{select, Receiver};
use lsp_server::{Connection, Message, Request, RequestId, Response};
use lsp_types::{
    request::GotoDefinition, GotoDefinitionResponse, InitializeParams, ServerCapabilities,
};

use crate::{cli::flags, from_json, server::Server, server_config::ServerConfig, Result};
use event::Event;

pub fn run_server(
    server_config: ServerConfig,
    connection: Connection,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut server = Server::new(connection.sender, server_config);
    assert_projects_exist(&mut server);
    fetch_workspaces(&mut server);
    run_event_loop(&mut server, connection.receiver)
}

fn assert_projects_exist(server: &mut Server) {
    if server.config.linked_projects().is_empty() && server.config.detached_files().is_empty() {
        server.sender.show_message(
            lsp_types::MessageType::ERROR,
            "rust-analyzer failed to discover workspace".to_string(),
        );
    };
}

fn fetch_workspaces(server: &mut Server) {
    todo!()
}

fn run_event_loop(server: &mut Server, inbox: Receiver<lsp_server::Message>) -> Result<()> {
    while let Some(event) = recv_next_event(server, &inbox) {
        if is_exit(&event) {
            return Ok(());
        }
        handle_event::handle_event(server, event)?
    }
    Err("client exited without proper shutdown sequence".into())
}

fn recv_next_event(server: &Server, inbox: &Receiver<lsp_server::Message>) -> Option<Event> {
    select! {
        recv(inbox) -> msg =>
            msg.ok().map(Event::Lsp),

        recv(server.taskpool.receiver) -> task =>
            Some(Event::Task(task.unwrap())),

        recv(server.vfs.loader.receiver) -> task =>
            Some(Event::Vfs(task.unwrap())),

        recv(server.flychecker.receiver) -> task =>
            Some(Event::Flycheck(task.unwrap())),
    }
}

fn is_exit(event: &Event) -> bool {
    todo!()
    // if let Event::Lsp(lsp_server::Message::Notification(not)) = &event {
    //     not.method == lsp_types::notification::Exit::METHOD
    // } else {
    //     false
    // }
}
