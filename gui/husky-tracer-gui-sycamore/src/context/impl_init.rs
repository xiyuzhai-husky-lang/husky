use futures::channel::mpsc::Receiver;

use super::*;

impl TracerContext {
    pub(super) fn init(&self, read: Receiver<HuskyTracerServerMessage>) {
        self.send_init_request();
        self.spawn_listening(read)
    }

    fn send_init_request(&self) {
        let mut gui_message_sender = self.ws.gui_message_sender.clone();
        let request_id = self.ws.issue_request_id();
        let this = self.clone();
        self.ws.send_request(
            HuskyTracerGuiMessageVariant::InitRequest,
            Some(Box::new(move |response| match response.variant {
                HuskyTracerServerMessageVariant::Init { init_data } => {
                    this.receive_init_data(init_data)
                }
                _ => panic!(),
            })),
        );
    }

    fn receive_init_data(&self, init_data: InitData) {
        self.tree_context
            .init(&init_data.focus, init_data.trace_init_data);
        self.figure_context
            .init(init_data.figures, init_data.figure_controls);
        self.focus_context.init(init_data.focus)
    }

    fn spawn_listening(&self, mut read: Receiver<HuskyTracerServerMessage>) {
        let this = self.clone();
        spawn_local({
            let context = this.clone();
            async move {
                while let Some(notif) = read.next().await {
                    context.handle_server_notification(notif)
                }
                log::debug!("WebSocket Closed");
            }
        });
    }

    pub(super) fn handle_server_message_str(&self, server_message_str: &str) {
        self.handle_server_notification(serde_json::from_str(server_message_str).unwrap())
    }

    fn handle_server_notification(&self, server_message: HuskyTracerServerMessage) {
        assert!(server_message.opt_request_id.is_none());
        match server_message.variant {
            // HuskyTracerServerMessageVariant::Init { init_data } => {
            //     self.receive_init_data(init_data)
            // }
            // HuskyTracerServerMessageVariant::Trace { trace_props } => todo!(),
            // HuskyTracerServerMessageVariant::DecodeFocus { focus_result } => todo!(),
            // HuskyTracerServerMessageVariant::LockFocus {
            //     focus,
            //     opt_active_trace_id_for_figure,
            //     opt_figure,
            //     opt_figure_control,
            // } => todo!(),
            // HuskyTracerServerMessageVariant::TraceStalk { stalk } => todo!(),
            _ => panic!(),
        }
    }
}
