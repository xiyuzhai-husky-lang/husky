use super::*;

pub fn handle_message(
    debugger: Arc<Debugger>,
    text: &str,
    client_sender: UnboundedSender<Result<Message, warp::Error>>,
) {
    if debugger.config.verbose {
        p!(text);
    }
    match serde_json::from_str(text) {
        Ok::<Query, _>(query) => {
            let debugger_ = debugger.clone();
            let client_sender_ = client_sender.clone();
            let future = async move {
                match client_sender_.send(Ok(Message::text(
                    serde_json::to_string(&match query {
                        Query::Activate {
                            id,
                            opt_focus_for_figure,
                        } => {
                            debugger_.activate(id).await;
                            let (opt_figure, opt_figure_control) =
                                if let Some(ref focus) = opt_focus_for_figure {
                                    let runtime = &mut debugger_.runtime.lock().unwrap();
                                    let trace = runtime.trace(id);
                                    (
                                        Some(runtime.figure(id, focus)),
                                        Some(runtime.figure_control(&trace, focus)),
                                    )
                                } else {
                                    (None, None)
                                };
                            Response::Activate {
                                id,
                                opt_focus_for_figure,
                                opt_figure,
                                opt_figure_control,
                            }
                        }
                        Query::ToggleExpansion {
                            id,
                            effective_opt_input_id,
                            request_subtraces,
                        } => {
                            debugger_.toggle_expansion(id).await;
                            let (opt_subtraces, associated_traces) = if request_subtraces {
                                let subtraces =
                                    debugger_.subtraces(id, effective_opt_input_id).await;
                                let mut associated_traces = vec![];
                                subtraces.iter().for_each(|trace| {
                                    trace.collect_associated_traces(&mut associated_traces)
                                });
                                (Some(subtraces), associated_traces)
                            } else {
                                (None, vec![])
                            };
                            Response::ToggleExpansion {
                                id,
                                effective_opt_input_id,
                                opt_subtraces,
                                associated_traces,
                            }
                        }
                        Query::ToggleShow { id } => {
                            debugger_.toggle_show(id).await;
                            Response::ToggleShow { id }
                        }
                        Query::Trace { id } => {
                            let trace = debugger_.trace(id);
                            Response::Trace { id, trace }
                        }
                        Query::TraceStalk { trace_id, input_id } => {
                            let stalk = debugger_.trace_stalk(trace_id, input_id).await;
                            Response::TraceStalk {
                                trace_id,
                                input_id,
                                stalk,
                            }
                        }
                        Query::DecodeFocus { ref command } => {
                            let focus_result = debugger_.decode_focus(command);
                            Response::DecodeFocus { focus_result }
                        }
                        Query::LockFocus {
                            focus,
                            opt_active_trace_id_for_figure,
                        } => {
                            let (opt_figure, opt_figure_control) =
                                if let Some(trace_id) = opt_active_trace_id_for_figure {
                                    let runtime = &mut debugger_.runtime.lock().unwrap();
                                    let trace = runtime.trace(trace_id);
                                    (
                                        Some(runtime.figure(trace_id, &focus)),
                                        Some(runtime.figure_control(&trace, &focus)),
                                    )
                                } else {
                                    (None, None)
                                };
                            Response::LockFocus {
                                focus,
                                opt_active_trace_id_for_figure,
                                opt_figure,
                                opt_figure_control,
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
