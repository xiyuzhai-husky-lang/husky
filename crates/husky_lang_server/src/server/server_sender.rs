use crossbeam_channel::{unbounded, Receiver, Sender};

pub(crate) struct ServerSender {
    sender: Sender<lsp_server::Message>,
}

impl ServerSender {
    pub(super) fn new(sender: Sender<lsp_server::Message>) -> ServerSender {
        ServerSender { sender }
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

    pub(crate) fn show_message(&mut self, typ: lsp_types::MessageType, message: String) {
        let message = message;
        self.send_notification::<lsp_types::notification::ShowMessage>(
            lsp_types::ShowMessageParams { typ, message },
        )
    }
}
