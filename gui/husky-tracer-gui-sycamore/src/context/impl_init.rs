use super::*;

impl DebuggerContext {
    pub(super) fn request_init(&mut self) {
        let mut gui_message_sender = self.ws.gui_message_sender.clone();
        let request_id = self.ws.issue_request_id();
        self.call_backs.insert(
            request_id,
            Box::new(|this, response| match response.variant {
                DebuggerServerMessageVariant::Init { init_data } => this.init(init_data),
                _ => panic!(),
            }),
        );
        spawn_local({
            async move {
                log::info!("send init request");
                gui_message_sender
                    .send(
                        serde_json::to_string(&DebuggerGuiMessage {
                            opt_request_id: Some(request_id),
                            variant: DebuggerGuiMessageVariant::InitRequest,
                        })
                        .unwrap(),
                    )
                    .await;
            }
        });
    }

    pub(super) fn init(&mut self, init_data: InitData) {
        self.trace_context
            .init(&init_data.focus, init_data.trace_init_data);
        self.figure_context
            .init(init_data.figures, init_data.figure_controls);
        self.focus_context.init(init_data.focus)
    }
}
