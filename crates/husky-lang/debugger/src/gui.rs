mod query;
mod response;
mod tests;

use crate::*;
use common::*;
use futures::{task::SpawnExt, FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, UnboundedSender};
use trace::{FigureProps, Trace, TraceId, TraceStalk};
use warp::ws::{Message, WebSocket};

use query::Query;
use response::Response;

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
    init_gui(&debugger, client_sender.clone());
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
                                Query::Subtraces {
                                    trace_id: id,
                                    opt_input_id: input_locked_on,
                                } => {
                                    let subtraces = debugger_.subtraces(id, input_locked_on).await;
                                    Response::Subtraces {
                                        id,
                                        input_locked_on,
                                        subtraces,
                                    }
                                }
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
                                Query::LockInput { input_str } => {
                                    let (input_locked_on, message) =
                                        debugger_.lock_input(input_str).await;
                                    Response::DidLockInput {
                                        input_locked_on,
                                        message,
                                    }
                                }
                                Query::TraceStalk { trace_id, input_id } => {
                                    let stalk = debugger_.trace_stalk(trace_id, input_id).await;
                                    Response::TraceStalk {
                                        trace_id,
                                        input_id,
                                        stalk,
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

fn init_gui(debugger: &Debugger, sender: UnboundedSender<Result<Message, warp::Error>>) {
    let root_traces = debugger.root_traces();
    let expansions = debugger.expansions();
    let showns = debugger.showns();
    let state = debugger.state.lock().unwrap();
    let runtime = debugger.runtime.lock().unwrap();
    epin!();
    let traces = runtime.traces();
    epin!();
    let response = Response::Init {
        active_trace_id: state.active_trace_id,
        opt_input_id: debugger.input_id(),
        traces,
        root_traces: &root_traces,
        expansions: &expansions,
        showns: &showns,
    };
    epin!();
    match sender.send(Ok(Message::text(serde_json::to_string(&response).unwrap()))) {
        Ok(_) => {
            println!("init message sent")
        }
        Err(_) => todo!(),
    };
}
