use std::cell::Cell;

use crate::*;
use futures::{channel::mpsc::Sender, stream::SplitStream, SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;

#[derive(Clone)]
pub struct WebsocketService {
    pub gui_message_sender: Sender<String>,
    next_request_id: Cell<usize>,
}

impl std::fmt::Debug for WebsocketService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebsocketService")
            .field("tx", &self.gui_message_sender)
            .finish()
    }
}

impl WebsocketService {
    pub fn new() -> (Self, SplitStream<WebSocket>) {
        let ws = WebSocket::open("ws://127.0.0.1:51617/query").unwrap();

        let (mut write, read) = ws.split();

        let (mut gui_message_sender, mut gui_message_receiver) =
            futures::channel::mpsc::channel::<String>(1000);

        spawn_local(async move {
            while let Some(s) = gui_message_receiver.next().await {
                // log::debug!("got event from channel! {}", s);
                write.send(Message::Text(s)).await.unwrap();
            }
        });

        (
            Self {
                gui_message_sender,
                next_request_id: Cell::new(0),
            },
            read,
        )
    }

    pub fn issue_request_id(&self) -> usize {
        let request_id = self.next_request_id.get();
        self.next_request_id.set(request_id + 1);
        request_id
    }
}
