mod dispatch;
mod event;
mod handle_event;

use std::error::Error;

use crossbeam_channel::Receiver;
use lsp_types::notification::Notification as _;

use crate::{config::ServerConfig, server::Server, Result};
use event::Event;

pub fn run_server(
    server_config: ServerConfig,
    connection: lsp_server::Connection,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut server = Server::new(connection.sender, server_config);
    check_projects_exist(&mut server);
    return run_event_loop(&mut server, connection.receiver);

    fn check_projects_exist(server: &mut Server) {
        if server.config.projects.is_empty() {
            server.comm.show_message(
                lsp_types::MessageType::ERROR,
                "husky-lang-server failed to discover workspace".to_string(),
            );
        };
    }

    fn run_event_loop(server: &mut Server, inbox: Receiver<lsp_server::Message>) -> Result<()> {
        while let Some(event) = inbox.recv().ok().map(Event::Lsp) {
            if is_exit(&event) {
                return Ok(());
            }
            handle_event::handle_event(server, event)?
        }
        return Err("client exited without proper shutdown sequence".into());

        fn is_exit(event: &Event) -> bool {
            if let Event::Lsp(lsp_server::Message::Notification(not)) = &event {
                not.method == lsp_types::notification::Exit::METHOD
            } else {
                false
            }
        }
    }
}
