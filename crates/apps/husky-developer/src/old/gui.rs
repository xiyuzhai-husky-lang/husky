mod handle;
mod tests;

use crate::*;
use futures::{FutureExt, StreamExt};
use handle::handle_message;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

pub(crate) async fn handle_query(
    socket: warp::ws::Ws,
    server: Arc<HuskyDeveloperInstance>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(socket.on_upgrade(move |ws| handle_query_upgraded(ws, server)))
}

pub(crate) async fn handle_query_upgraded(websocket: WebSocket, dev: Arc<HuskyDeveloperInstance>) {
    let (tx, mut rx) = websocket.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel::<Result<Message, warp::Error>>();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(tx).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));
    println!(
        "{}husky:{} query connection established.",
        husky_print_utils::CYAN,
        husky_print_utils::RESET
    );
    let mut gui_messages = Vec::new();
    while let Some(message_result) = rx.next().await {
        let message = message_result.expect("error receiving ws message: {}");
        match message.to_str() {
            Ok(text) => match serde_json::from_str(text) {
                Ok::<HuskyTracerGuiMessage, _>(gui_message) => {
                    println!(
                        r#"receive message:
    {text}"#
                    );
                    gui_messages.push(gui_message);
                    let now = Instant::now();
                    handle_message(dev.clone(), client_sender.clone(), &gui_messages).unwrap();
                    println!(
                        "{} milliseconds elapsed for handling message",
                        now.elapsed().as_millis(),
                    );
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
                        husky_print_utils::CYAN,
                        husky_print_utils::RESET
                    );
                } else {
                    eprintln!("nontext message received: {:?}", message);
                }
            }
        };
    }
}
