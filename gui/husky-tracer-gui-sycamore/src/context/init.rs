use futures::channel::mpsc::Receiver;
use web_sys::KeyboardEvent;

use super::impl_status_change::StatusChange;

use super::*;

impl DeveloperGuiContext {
    pub(super) fn init<'a>(&'static self, read: SplitStream<WebSocket>) {
        self.ws.init(read, self);
        self.add_event_listeners_to_dialogues();
        self.send_init_request();
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
            HuskyTracerGuiMessageVariant::HotReloadRequest,
            true,
            || unreachable!(),
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
