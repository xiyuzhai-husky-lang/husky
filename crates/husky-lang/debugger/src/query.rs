use crate::*;
use common::*;
use futures::{task::SpawnExt, FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use trace::{FigureProps, Trace, TraceId};
use warp::ws::{Message, WebSocket};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
enum Query {
    RootTraces,
    Subtraces { id: TraceId },
    Figure { id: TraceId },
    Activate { id: TraceId },
    ToggleExpansion { id: TraceId },
    ToggleShow { id: TraceId },
    Trace { id: TraceId },
    LockInput { input_temp: String },
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
        id: TraceId,
        subtraces: Arc<Vec<Arc<Trace>>>,
    },
    Figure {
        id: TraceId,
        figure: Option<FigureProps>,
    },
    DidActivate {
        id: TraceId,
    },
    DidToggleExpansion {
        id: TraceId,
    },
    DidToggleShow {
        id: TraceId,
    },
    Trace {
        id: TraceId,
        trace: Arc<Trace>,
    },
    DidLockInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        input_locked_on: Option<Option<usize>>,
        message: Option<String>,
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
        "{}husky:{} query connection established.",
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
                                Query::ToggleShow { id } => {
                                    debugger_.toggle_show(id).await;
                                    Response::DidToggleShow { id }
                                }
                                Query::Figure { id } => Response::Figure {
                                    id,
                                    figure: debugger_.figure(id).await,
                                },
                                Query::Trace { id } => {
                                    let trace = debugger_.trace(id).await;
                                    Response::Trace { id, trace }
                                }
                                Query::LockInput { input_temp } => {
                                    let (input_locked_on, message) =
                                        debugger_.lock_input(input_temp).await;
                                    Response::DidLockInput {
                                        input_locked_on,
                                        message,
                                    }
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
                Err(_) => {
                    p!(text);
                    todo!()
                }
            },
            Err(_) => {
                if msg.is_close() {
                    println!(
                        "{}husky:{} query connection closed.",
                        common::show::CYAN,
                        common::show::RESET
                    );
                } else {
                    eprintln!("nontext message received: {:?}", msg);
                }
            }
        };
    }
}
