use std::time::Instant;

use crossbeam_channel::Sender;

use super::Server;

pub(crate) struct Communicator {
    pub(crate) sender: Sender<lsp_server::Message>,
    pub(crate) req_queue: ReqQueue,
}

pub(crate) type ReqHandler = fn(&mut Server, lsp_server::Response);
pub(crate) type ReqQueue = lsp_server::ReqQueue<(String, Instant), ReqHandler>;

impl Communicator {
    pub(super) fn new(sender: Sender<lsp_server::Message>) -> Communicator {
        Communicator {
            sender,
            req_queue: ReqQueue::default(),
        }
    }
    fn send(&self, message: lsp_server::Message) {
        self.sender.send(message).unwrap()
    }

    pub(crate) fn send_notification<N: lsp_types::notification::Notification>(
        &self,
        params: N::Params,
    ) {
        let not = lsp_server::Notification::new(N::METHOD.to_string(), params);
        self.send(not.into());
    }

    pub(crate) fn _send_request<R: lsp_types::request::Request>(
        &mut self,
        params: R::Params,
        handler: ReqHandler,
    ) {
        let request = self
            .req_queue
            .outgoing
            .register(R::METHOD.to_string(), params, handler);
        self.send(request.into());
    }

    pub(crate) fn respond(&mut self, response: lsp_server::Response) {
        if let Some((method, start)) = self.req_queue.incoming.complete(response.id.clone()) {
            if let Some(err) = &response.error {
                if err.message.starts_with("server panicked") {
                    self.poke_husky_developer(format!("{}, check the log", err.message))
                }
            }

            let duration = start.elapsed();
            tracing::info!(
                "handled {} - ({}) in {:0.2?}",
                method,
                response.id,
                duration
            );
            self.send(response.into());
        }
    }

    pub(crate) fn show_message(&mut self, typ: lsp_types::MessageType, message: String) {
        let message = message;
        self.send_notification::<lsp_types::notification::ShowMessage>(
            lsp_types::ShowMessageParams { typ, message },
        )
    }

    pub(crate) fn poke_husky_developer(&mut self, message: String) {
        self.show_message(lsp_types::MessageType::ERROR, message)
    }
}
