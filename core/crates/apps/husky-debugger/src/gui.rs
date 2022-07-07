mod handle;
mod tests;

use std::panic::catch_unwind;

use crate::*;
use futures::{task::SpawnExt, FutureExt, StreamExt};
use handle::handle_message;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

pub(crate) async fn handle_query(
    socket: warp::ws::Ws,
    server: Arc<HuskyDebugger>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(socket.on_upgrade(move |ws| handle_query_upgraded(ws, server)))
}

pub(crate) async fn handle_query_upgraded(websocket: WebSocket, debugger: Arc<HuskyDebugger>) {
    let (tx, mut rx) = websocket.split();
    let (client_sender, mut client_rcv) = mpsc::unbounded_channel::<Result<Message, warp::Error>>();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(
        client_rcv.forward(tx).map(|result| {
            if let Err(e) = result {
                eprintln!("error sending websocket msg: {}", e);
            }
        }), // async move {
            // while let Some(_) = client_rcv.recv().await {
            //     //   tx.
            //     todo!()
            // }
    );
    println!(
        "{}husky:{} query connection established.",
        print_utils::CYAN,
        print_utils::RESET
    );
    while let Some(message_result) = rx.next().await {
        let message = message_result.expect("error receiving ws message: {}");
        let mut gui_messages = Vec::new();
        match message.to_str() {
            Ok(text) => match serde_json::from_str(text) {
                Ok::<HuskyTracerGuiMessage, _>(gui_message) => {
                    gui_messages.push(gui_message);
                    handle_message(debugger.clone(), client_sender.clone(), &gui_messages)
                }
                Err(_) => {
                    p!(text);
                    todo!()
                }
            },
            Err(_) => {
                if message.is_close() {
                    println!(
                        "{}husky:{} query connection closed.",
                        print_utils::CYAN,
                        print_utils::RESET
                    );
                } else {
                    eprintln!("nontext message received: {:?}", message);
                }
            }
        };
    }
}
