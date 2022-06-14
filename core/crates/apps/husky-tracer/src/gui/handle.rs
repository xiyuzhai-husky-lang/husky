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
            let msg = HuskyTracerServerMessage {
                opt_request_id,
                variant,
            };
            match serde_json::to_string(&msg) {
                Ok(text) => Some(text),
                Err(e) => {
                    p!(msg);
                    p!(e);
                    todo!()
                }
            }
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
            HuskyTracerGuiMessageVariant::InitDataRequest => Some(self.trace_time.init_state()),
            HuskyTracerGuiMessageVariant::Activate {
                trace_id,
                opt_focus_for_figure,
            } => {
                self.trace_time.activate(trace_id);
                should_eq!(
                    request.opt_request_id.is_some(),
                    opt_focus_for_figure.is_some()
                );
                if let Some(ref focus) = opt_focus_for_figure {
                    Some(HuskyTracerServerMessageVariant::Activate {
                        figure_canvas_data: self.trace_time.figure_canvas_data(trace_id, focus),
                        figure_control_data: self.trace_time.figure_control(trace_id, focus),
                    })
                } else {
                    None
                }
            }
            HuskyTracerGuiMessageVariant::ToggleExpansion { trace_id } => {
                if let Some((new_traces, subtrace_ids)) = self.trace_time.toggle_expansion(trace_id)
                {
                    Some(HuskyTracerServerMessageVariant::ToggleExpansion {
                        new_traces,
                        subtrace_ids,
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
            HuskyTracerGuiMessageVariant::UpdateFigureControlData {
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
