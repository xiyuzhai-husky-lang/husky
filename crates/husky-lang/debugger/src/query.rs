use crate::*;
use common::*;
use futures::{task::SpawnExt, FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use trace::{FigureProps, Trace};
use warp::ws::{Message, WebSocket};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
enum Query {
    RootTraces,
    Subtraces { id: usize },
    Activate { id: usize },
    ToggleExpansion { id: usize },
    Figure { id: usize },
}

#[test]
fn print_queries() {
    p!(serde_json::to_string(&Query::RootTraces));
    let query: Query = serde_json::from_str("\"RootTraces\"").unwrap();
    should_eq!(query, Query::RootTraces);
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type")]
pub enum Response {
    RootTraces {
        root_traces: Arc<Vec<Arc<Trace>>>,
    },
    Subtraces {
        id: usize,
        subtraces: Arc<Vec<Arc<Trace>>>,
    },
    Figure {
        id: usize,
        figure: Option<FigureProps>,
    },
    DidActivate {
        id: usize,
    },
    DidToggleExpansion {
        id: usize,
    },
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
    println!(
        "{}query connection established.{}",
        common::show::CYAN,
        common::show::RESET
    );

    while let Some(result) = rx.next().await {
        let msg = result.expect("error receiving ws message: {}");
        match msg.to_str() {
            Ok(text) => match serde_json::from_str(text) {
                Ok::<Query, _>(query) => {
                    let debugger_ = debugger.clone();
                    let client_sender_ = client_sender.clone();
                    let future = async move {
                        match client_sender_.send(Ok(Message::text(
                            serde_json::to_string(&match query {
                                Query::RootTraces => Response::RootTraces {
                                    root_traces: debugger_.root_traces().await,
                                },
                                Query::Subtraces { id } => Response::Subtraces {
                                    id,
                                    subtraces: debugger_.subtraces(id).await,
                                },
                                Query::Activate { id } => {
                                    debugger_.activate(id).await;
                                    Response::DidActivate { id }
                                }
                                Query::ToggleExpansion { id } => {
                                    debugger_.toggle_expansion(id).await;
                                    Response::DidToggleExpansion { id }
                                }
                                Query::Figure { id } => Response::Figure {
                                    id,
                                    figure: debugger_.figure(id).await,
                                },
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
                if msg.is_close() {
                    println!(
                        "{}query connection closed.{}",
                        common::show::RED,
                        common::show::RESET
                    );
                } else {
                    eprintln!("nontext message received: {:?}", msg);
                }
            }
        };
    }
}
