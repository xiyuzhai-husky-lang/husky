use std::time::Instant;

use crossbeam_channel::{unbounded, Receiver, Sender};

use super::Server;

pub(crate) struct ServerSender {
    sender: Sender<lsp_server::Message>,
    req_queue: ReqQueue,
}

pub(crate) type ReqHandler = fn(&mut Server, lsp_server::Response);
pub(crate) type ReqQueue = lsp_server::ReqQueue<(String, Instant), ReqHandler>;

impl ServerSender {
    pub(super) fn new(sender: Sender<lsp_server::Message>) -> ServerSender {
        ServerSender {
            sender,
            req_queue: ReqQueue::default(),
        }
    }
    fn send(&mut self, message: lsp_server::Message) {
        self.sender.send(message).unwrap()
    }

    pub(crate) fn send_notification<N: lsp_types::notification::Notification>(
        &mut self,
        params: N::Params,
    ) {
        let not = lsp_server::Notification::new(N::METHOD.to_string(), params);
        self.send(not.into());
    }

    pub(crate) fn send_request<R: lsp_types::request::Request>(
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

    pub(crate) fn show_message(&mut self, typ: lsp_types::MessageType, message: String) {
        let message = message;
        self.send_notification::<lsp_types::notification::ShowMessage>(
            lsp_types::ShowMessageParams { typ, message },
        )
    }
}
