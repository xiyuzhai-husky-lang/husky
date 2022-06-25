mod handle;
mod tests;

use crate::*;
use futures::{task::SpawnExt, FutureExt, StreamExt};
use handle::handle_message;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, UnboundedSender};
use warp::ws::{Message, WebSocket};

pub(crate) async fn handle_query(
    socket: warp::ws::Ws,
    server: Arc<HuskyDebugger>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(socket.on_upgrade(move |ws| handle_query_upgraded(ws, server)))
}

pub(crate) async fn handle_query_upgraded(websocket: WebSocket, debugger: Arc<HuskyDebugger>) {
    let (tx, mut rx) = websocket.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel::<Result<Message, warp::Error>>();
    tokio::task::spawn(client_rcv.forward(tx).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));
    println!(
        "{}husky:{} query connection established.",
        print_utils::CYAN,
        print_utils::RESET
    );
    while let Some(message_result) = rx.next().await {
        let message = message_result.expect("error receiving ws message: {}");
        match message.to_str() {
            Ok(text) => handle_message(debugger.clone(), text, client_sender.clone()),
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
