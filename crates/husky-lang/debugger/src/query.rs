use std::borrow::Cow;

use crate::*;
use common::*;
use futures::{task::SpawnExt, FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use trace::Trace;
use warp::ws::{Message, WebSocket};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
enum Query {
    Dummy,
    MainTrace,
    RootTraces,
}

#[test]
fn print_queries() {
    p!(serde_json::to_string(&Query::RootTraces));
    let query: Query = serde_json::from_str("\"RootTraces\"").unwrap();
    should_eq!(query, Query::RootTraces);
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "t", content = "c")]
pub enum Response {
    MainTrace(()),
    Dummy,
    RootTraces(Cow<'static, [Trace]>),
}

pub(crate) async fn handle_query(
    socket: warp::ws::Ws,
    server: Arc<Debugger>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(socket.on_upgrade(move |ws| handle_query_upgraded(ws, server)))
}

pub(crate) async fn handle_query_upgraded(websocket: WebSocket, debugger: Arc<Debugger>) {
    let (tx, mut rx) = websocket.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel::<Result<Message, warp::Error>>();
    tokio::task::spawn(client_rcv.forward(tx).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));
    println!("query connection established.");

    while let Some(result) = rx.next().await {
        let msg = result.expect("error receiving ws message: {}");
        p!(msg);
        match msg.to_str() {
            Ok(text) => match serde_json::from_str(text) {
                Ok::<Query, _>(query) => {
                    let debugger_ = debugger.clone();
                    let client_sender_ = client_sender.clone();
                    let future = async move {
                        match client_sender_.send(Ok(Message::text(
                            serde_json::to_string(&match query {
                                Query::MainTrace => {
                                    Response::MainTrace(debugger_.runtime.main_trace())
                                }
                                Query::Dummy => Response::Dummy,
                                Query::RootTraces => {
                                    Response::RootTraces(debugger_.runtime.root_traces())
                                }
                            })
                            .unwrap(),
                        ))) {
                            Ok(_) => (),
                            Err(_) => todo!(),
                        }
                    };
                    debugger.threadpool.spawn(future).unwrap();
                }
                Err(_) => todo!(),
            },
            Err(_) => {
                eprintln!("nontext message received: {:?}", msg);
            }
        };
    }
}
