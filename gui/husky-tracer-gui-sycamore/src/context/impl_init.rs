use futures::channel::mpsc::Receiver;
use web_sys::KeyboardEvent;

use super::impl_status_change::StatusChange;

use super::*;

impl DebuggerContext {
    pub(super) fn init<'a>(&'static self, read: Receiver<HuskyTracerServerMessage>) {
        self.add_event_listeners_to_dialogues();
        self.send_init_request();
        self.spawn_listening(read)
    }

    fn add_event_listeners_to_dialogues(&'static self) {
        let dialog = restriction_dialog();
        add_event_listener!(dialog, "keydown", move |event: web_sys::UiEvent| {
            let event: KeyboardEvent = event.unchecked_into();
            match event.key().as_str() {
                "Enter" => {
                    let sample_id_value = sample_id_input().value();
                    match sample_id_value.parse::<usize>() {
                        Ok(raw) => {
                            restriction_dialog().close();
                            log::info!(
                                "restriction_dialog().open() = {}",
                                restriction_dialog().open()
                            );
                            log::info!("set sample id to {raw}");
                            self.handle_status_change(StatusChange::update_restriction(
                                self,
                                |res| res.set_sample_id(SampleId(raw)),
                            ))
                        }
                        Err(_) => alert!("`{}` is not a valid sample id", sample_id_value),
                    }
                }
                _ => (),
            }
        });
    }

    fn send_init_request(&'static self) {
        let mut gui_message_sender = self.ws.gui_message_sender.clone();
        let request_id = self.ws.issue_request_id();
        self.ws.send_message(
            HuskyTracerGuiMessageVariant::InitDataRequest,
            Some(Box::new(move |response| match response.variant {
                HuskyTracerServerMessageVariant::Init { init_data } => {
                    self.receive_init_data(init_data)
                }
                _ => panic!(),
            })),
        );
    }

    fn receive_init_data<'a>(&'static self, init_data: InitData) {
        if init_data.trace_init_data.opt_active_trace_id.is_some() {
            assert!(init_data.figure_canvases.len() > 0);
        }
        // order matters
        self.restriction_context.init(init_data.restriction.clone());
        self.figure_context.init(
            self.alloc_key_value_pairs(init_data.figure_canvases)
                .collect(),
            self.alloc_key_signal_pairs(init_data.figure_controls)
                .collect(),
            init_data.pins,
        );
        self.trace_context.init(
            init_data
                .trace_init_data
                .trace_nodes
                .into_iter()
                .map(|trace_node| TraceNodeState::from_data(self.scope, trace_node))
                .collect(),
            init_data
                .trace_init_data
                .trace_stalks
                .into_iter()
                .map(|(k, v)| (k, self.alloc_value(v)))
                .collect(),
            init_data
                .trace_init_data
                .trace_statss
                .into_iter()
                .map(|(k, v)| (k, v.map(|v| self.alloc_value(v))))
                .collect(),
            init_data
                .trace_init_data
                .subtrace_ids_map
                .into_iter()
                .map(|(k, v)| (k, self.alloc_value(v) as &'static [TraceId]))
                .collect(),
            init_data.trace_init_data.root_trace_ids,
            init_data.trace_init_data.opt_active_trace_id,
            init_data.restriction.opt_sample_id(),
        );
    }

    fn spawn_listening(&'static self, mut read: Receiver<HuskyTracerServerMessage>) {
        spawn_local({
            async move {
                while let Some(notif) = read.next().await {
                    self.handle_server_notification(notif)
                }
                log::debug!("WebSocket Closed");
            }
        });
    }

    pub(super) fn handle_server_message_str(&'static self, server_message_str: &str) {
        self.handle_server_notification(serde_json::from_str(server_message_str).unwrap())
    }

    fn handle_server_notification(&'static self, server_message: HuskyTracerServerMessage) {
        assert!(server_message.opt_request_id.is_none());
        match server_message.variant {
            _ => panic!(),
        }
    }
}
