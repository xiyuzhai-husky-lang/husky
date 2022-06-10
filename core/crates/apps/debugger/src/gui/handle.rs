use check_utils::should_eq;
use wild_utils::ref_to_mut_ref;

use super::*;

pub fn handle_message(
    debugger: Arc<Debugger>,
    text: &str,
    client_sender: UnboundedSender<Result<Message, warp::Error>>,
) {
    match serde_json::from_str(text) {
        Ok::<DebuggerGuiMessage, _>(request) => {
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

impl Debugger {
    async fn handle_gui_message(
        self: Arc<Self>,
        gui_message: DebuggerGuiMessage,
    ) -> Option<String> {
        let opt_request_id = gui_message.opt_request_id;
        let internal: &mut DebuggerInternal = &mut self.internal.lock().unwrap();
        let opt_response_variant = internal.handle_gui_message(gui_message);
        should_eq!(opt_request_id.is_some(), opt_response_variant.is_some());
        if let Some(variant) = opt_response_variant {
            Some(
                serde_json::to_string(&DebuggerServerMessage {
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

impl DebuggerInternal {
    fn handle_gui_message(
        &mut self,
        request: DebuggerGuiMessage,
    ) -> Option<DebuggerServerMessageVariant> {
        match request.variant {
            DebuggerGuiMessageVariant::InitRequest => Some(self.init_state()),
            DebuggerGuiMessageVariant::Activate {
                trace_id: id,
                opt_focus_for_figure,
            } => {
                self.activate(id);
                should_eq!(
                    request.opt_request_id.is_some(),
                    opt_focus_for_figure.is_some()
                );
                if let Some(ref focus) = opt_focus_for_figure {
                    let trace = self.runtime.trace(id);
                    Some(DebuggerServerMessageVariant::Activate {
                        figure_props: self.runtime.figure(id, focus),
                        figure_control_props: self.runtime.figure_control(&trace, focus),
                    })
                } else {
                    None
                }
            }
            DebuggerGuiMessageVariant::ToggleExpansion {
                trace_id: id,
                effective_opt_input_id,
                request_subtraces,
            } => {
                self.toggle_expansion(id);
                if request_subtraces {
                    let subtraces = self.subtraces(id, effective_opt_input_id);
                    let mut associated_traces = vec![];
                    subtraces
                        .iter()
                        .for_each(|trace| trace.collect_associated_traces(&mut associated_traces));
                    Some(DebuggerServerMessageVariant::ToggleExpansion {
                        effective_opt_input_id,
                        subtraces: subtraces.iter().map(|trace| trace.props.clone()).collect(),
                        associated_traces,
                    })
                } else {
                    None
                }
            }
            DebuggerGuiMessageVariant::ToggleShow { trace_id } => {
                self.toggle_show(trace_id);
                None
            }
            DebuggerGuiMessageVariant::Trace { id } => {
                let trace = self.trace(id);
                Some(DebuggerServerMessageVariant::Trace {
                    trace_props: trace.props.clone(),
                })
            }
            DebuggerGuiMessageVariant::TraceStalk { trace_id, input_id } => {
                let stalk = (*self.trace_stalk(trace_id, input_id)).clone();
                Some(DebuggerServerMessageVariant::TraceStalk { stalk })
            }
            DebuggerGuiMessageVariant::DecodeFocus { ref command } => {
                let focus_result = self.decode_focus(command);
                Some(DebuggerServerMessageVariant::DecodeFocus { focus_result })
            }
            DebuggerGuiMessageVariant::LockFocus {
                focus,
                opt_active_trace_id_for_figure,
            } => {
                let (opt_figure, opt_figure_control) =
                    if let Some(trace_id) = opt_active_trace_id_for_figure {
                        let trace = self.runtime.trace(trace_id);
                        (
                            Some(self.runtime.figure(trace_id, &focus)),
                            Some(self.runtime.figure_control(&trace, &focus)),
                        )
                    } else {
                        (None, None)
                    };
                Some(DebuggerServerMessageVariant::LockFocus {
                    focus,
                    opt_active_trace_id_for_figure,
                    opt_figure,
                    opt_figure_control,
                })
            }
            DebuggerGuiMessageVariant::UpdateFigureControlProps {
                trace_id,
                ref focus,
                figure_control_props,
            } => {
                self.runtime
                    .update_figure_control(trace_id, focus, figure_control_props);
                None
            }
        }
    }

    fn init_state(&mut self) -> DebuggerServerMessageVariant {
        let root_traces = self.runtime.root_traces();
        let expansions = self.expansions().clone();
        let showns = self.showns().clone();
        let state = &self.debug_time;
        let focus = self.runtime.focus();
        let mut figures = HashMap::default();
        let mut figure_controls = HashMap::default();
        let active_trace_id = state.active_trace_id;
        if let Some(active_trace_id) = active_trace_id {
            let active_trace = self.runtime.trace(active_trace_id);
            figures.insert(
                FigureKey::new(&active_trace.props),
                self.runtime.figure(active_trace_id, &focus),
            );
            figure_controls.insert(
                FigureControlKey::new(&active_trace.props),
                unsafe { ref_to_mut_ref(&self.runtime) }.figure_control(&active_trace, &focus),
            );
        }
        let traces = self.runtime.traces();
        DebuggerServerMessageVariant::Init {
            init_data: InitData {
                trace_init_data: TraceInitState {
                    active_trace_id,
                    traces,
                    subtraces_list: state.subtraces_map.clone(),
                    root_traces: (*root_traces).clone(),
                    expansions,
                    showns,
                },
                focus,
                figures,
                figure_controls,
            },
        }
    }
}
