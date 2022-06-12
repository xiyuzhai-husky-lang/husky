use check_utils::should_eq;
use wild_utils::ref_to_mut_ref;

use super::*;

pub fn handle_message(
    debugger: Arc<HuskyTracer>,
    text: &str,
    client_sender: UnboundedSender<Result<Message, warp::Error>>,
) {
    match serde_json::from_str(text) {
        Ok::<HuskyTracerGuiMessage, _>(request) => {
            let debugger_ = debugger.clone();
            let client_sender_ = client_sender.clone();
            let future = async move {
                if let Some(text) = debugger_.handle_gui_message(request).await {
                    match client_sender_.send(Ok(Message::text(text))) {
                        Ok(_) => (),
                        Err(_) => todo!(),
                    }
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

impl HuskyTracer {
    async fn handle_gui_message(
        self: Arc<Self>,
        gui_message: HuskyTracerGuiMessage,
    ) -> Option<String> {
        let opt_request_id = gui_message.opt_request_id;
        let internal: &mut HuskyTracerInternal = &mut self.internal.lock().unwrap();
        let opt_response_variant = internal.handle_gui_message(gui_message);
        should_eq!(opt_request_id.is_some(), opt_response_variant.is_some());
        if let Some(variant) = opt_response_variant {
            Some(
                serde_json::to_string(&HuskyTracerServerMessage {
                    opt_request_id,
                    variant,
                })
                .unwrap(),
            )
        } else {
            None
        }
    }
}

impl HuskyTracerInternal {
    fn handle_gui_message(
        &mut self,
        request: HuskyTracerGuiMessage,
    ) -> Option<HuskyTracerServerMessageVariant> {
        match request.variant {
            HuskyTracerGuiMessageVariant::InitRequest => Some(self.trace_time.init_state()),
            HuskyTracerGuiMessageVariant::Activate {
                trace_id: id,
                opt_focus_for_figure,
            } => {
                todo!()
                // self.trace_time.activate(id);
                // should_eq!(
                //     request.opt_request_id.is_some(),
                //     opt_focus_for_figure.is_some()
                // );
                // if let Some(ref focus) = opt_focus_for_figure {
                //     let trace = self.trace_time.trace(id);
                //     Some(HuskyTracerServerMessageVariant::Activate {
                //         figure_props: self.trace_time.figure(id, focus),
                //         figure_control_props: self.trace_time.figure_control(&trace, focus),
                //     })
                // } else {
                //     None
                // }
            }
            HuskyTracerGuiMessageVariant::ToggleExpansion {
                trace_id,
                request_subtraces,
            } => {
                self.trace_time.toggle_expansion(trace_id);
                if request_subtraces {
                    let subtraces = self.trace_time.subtraces(trace_id);
                    let mut associated_traces = vec![];
                    subtraces
                        .iter()
                        .for_each(|trace| trace.collect_associated_traces(&mut associated_traces));
                    Some(HuskyTracerServerMessageVariant::ToggleExpansion {
                        subtraces,
                        associated_traces,
                    })
                } else {
                    None
                }
            }
            HuskyTracerGuiMessageVariant::ToggleShow { trace_id } => {
                self.trace_time.toggle_show(trace_id);
                None
            }
            HuskyTracerGuiMessageVariant::Trace { id } => {
                let trace = self.trace_time.trace(id);
                Some(HuskyTracerServerMessageVariant::Trace {
                    trace_props: trace.props.clone(),
                })
            }
            HuskyTracerGuiMessageVariant::TraceStalk { trace_id, input_id } => {
                let stalk = (*self.trace_time.trace_stalk(trace_id, input_id)).clone();
                Some(HuskyTracerServerMessageVariant::TraceStalk { stalk })
            }
            HuskyTracerGuiMessageVariant::DecodeFocus { ref command } => {
                todo!()
                // let focus_result = self.trace_time.decode_focus(command);
                // Some(HuskyTracerServerMessageVariant::DecodeFocus { focus_result })
            }
            HuskyTracerGuiMessageVariant::LockFocus {
                focus,
                opt_active_trace_id_for_figure,
            } => {
                todo!()
                // let (opt_figure, opt_figure_control) =
                //     if let Some(trace_id) = opt_active_trace_id_for_figure {
                //         let trace = self.trace_time.trace(trace_id);
                //         (
                //             Some(self.trace_time.figure(trace_id, &focus)),
                //             Some(self.trace_time.figure_control(&trace, &focus)),
                //         )
                //     } else {
                //         (None, None)
                //     };
                // Some(HuskyTracerServerMessageVariant::LockFocus {
                //     focus,
                //     opt_active_trace_id_for_figure,
                //     opt_figure,
                //     opt_figure_control,
                // })
            }
            HuskyTracerGuiMessageVariant::UpdateFigureControlProps {
                trace_id,
                ref focus,
                figure_control_props,
            } => {
                self.trace_time
                    .update_figure_control(trace_id, focus, figure_control_props);
                None
            }
        }
    }
}
