use futures::{task::SpawnExt, StreamExt};
use serde::{Deserialize, Serialize};
use warp::ws::{WebSocket, Ws};

use crate::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Notification {
    DidChangeText,
}

pub(crate) async fn handle_notif(
    socket: Ws,
    server: Arc<HuskyDeveloperInstance>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(socket.on_upgrade(move |ws| handle_notif_upgraded(ws, server)))
}

pub(crate) async fn handle_notif_upgraded(
    websocket: WebSocket,
    server: Arc<HuskyDeveloperInstance>,
) {
    let (_, mut rx) = websocket.split();
    println!("upgraded");
    while let Some(result) = rx.next().await {
        let msg = result.expect("error receiving ws message: {}");
        let text = match msg.to_str() {
            Ok(notif) => notif,
            Err(_) => todo!(),
        };
        match serde_json::from_str(text) {
            Ok::<Notification, _>(notif) => {
                let server_ = server.clone();
                let future = async move {
                    match notif {
                        Notification::DidChangeText => server_.change_text(),
                    }
                };
                server.threadpool.spawn(future).unwrap();
            }
            Err(_) => todo!(),
        }
    }
    todo!()
}
