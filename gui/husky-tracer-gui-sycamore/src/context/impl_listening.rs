use super::*;

impl TracerContextInternal {
    pub(super) fn spawn_listening(this: Rc<Self>, mut read: SplitStream<WebSocket>) {
        spawn_local({
            let context = this.clone();
            async move {
                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(Message::Text(data)) => {
                            // log::debug!("from websocket: {}", data);
                            context.handle_server_message_str(&data)
                        }
                        Ok(Message::Bytes(b)) => {
                            let decoded = std::str::from_utf8(&b);
                            if let Ok(val) = decoded {
                                // log::debug!("from websocket: {}", val);
                                context.handle_server_message_str(val)
                            }
                        }
                        Err(e) => {
                            log::error!("")
                        }
                    }
                }
                log::debug!("WebSocket Closed");
            }
        });
    }
}
