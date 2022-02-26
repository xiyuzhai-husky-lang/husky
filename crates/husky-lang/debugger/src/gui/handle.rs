use super::*;

pub fn handle_message(
    debugger: Arc<Debugger>,
    text: &str,
    client_sender: UnboundedSender<Result<Message, warp::Error>>,
) {
    match serde_json::from_str(text) {
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
                            let (input_locked_on, message) = debugger_.lock_input(input_str).await;
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
    }
}
