use super::*;
use husky_check_utils::should_eq;
use husky_tracetime::TracetimeHotReloadR;
use monad::Monad;
use std::panic::catch_unwind;
use std::path::PathBuf;
use xxhash_rust::xxh3::xxh3_64;

pub enum HandleGuiMessageM<T> {
    Ok(T),
}

impl<T> Monad for HandleGuiMessageM<T> {}

impl<T> HandleGuiMessageM<T> {
    pub(crate) fn unwrap(self) -> T {
        match self {
            HandleGuiMessageM::Ok(t) => t,
        }
    }
}

impl<T> std::ops::FromResidual<DebuggerHotReloadR> for HandleGuiMessageM<T> {
    fn from_residual(residual: DebuggerHotReloadR) -> Self {
        todo!()
    }
}

pub struct HandleGuiMessageR;

impl<T> std::ops::FromResidual<HandleGuiMessageR> for HandleGuiMessageM<T> {
    fn from_residual(residual: HandleGuiMessageR) -> Self {
        unreachable!()
    }
}

impl<T> std::ops::Try for HandleGuiMessageM<T> {
    type Output = T;

    type Residual = HandleGuiMessageR;

    fn from_output(output: Self::Output) -> Self {
        HandleGuiMessageM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            HandleGuiMessageM::Ok(output) => std::ops::ControlFlow::Continue(output),
        }
    }
}

pub(crate) fn handle_message(
    debugger: Arc<HuskyDebuggerInstance>,
    client_sender: UnboundedSender<Result<Message, warp::Error>>,
    gui_messages: &[HuskyTracerGuiMessage],
    config: &HuskyDebuggerConfig,
) -> HandleGuiMessageM<()> {
    let debugger_ = debugger.clone();
    let client_sender_ = client_sender.clone();
    let latest_gui_message = gui_messages.last().unwrap();
    match catch_unwind(|| debugger_.handle_gui_message(&latest_gui_message)) {
        Ok(monad) => match monad? {
            Some(text) => match client_sender_.send(Ok(Message::text(text))) {
                Ok(_) => HandleGuiMessageM::Ok(()),
                Err(_) => todo!(),
            },
            None => HandleGuiMessageM::Ok(()),
        },
        Err(_) => HandleGuiMessageM::Ok(save_server_history(
            &(DebuggerServerHistory {
                config: config.clone(),
                gui_messages: gui_messages.to_vec(),
            }),
        )),
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct DebuggerServerHistory {
    config: HuskyDebuggerConfig,
    gui_messages: Vec<HuskyTracerGuiMessage>,
}

fn save_server_history(server_history: &DebuggerServerHistory) {
    let value = serde_json::to_string_pretty(server_history).unwrap();
    let filename = format!("history-{}.json", xxh3_64(value.as_bytes()));
    let filename: &str = &filename;
    let filepath: PathBuf = format!("tests/debugger/server-history/{filename}").into();
    husky_io_utils::diff_write(&filepath, &value, true)
}

impl HuskyDebuggerInstance {
    fn handle_gui_message(
        self: Arc<Self>,
        gui_message: &HuskyTracerGuiMessage,
    ) -> HandleGuiMessageM<Option<String>> {
        let internal: &mut HuskyDebuggerInternal = &mut self.internal.lock().unwrap();
        let opt_response_variant = internal.handle_gui_message(gui_message)?;
        should_eq!(
            gui_message.opt_request_id.is_some(),
            opt_response_variant.is_some(),
            "{:?}",
            gui_message
        );
        if let Some(variant) = opt_response_variant {
            let msg = HuskyTracerServerMessage {
                opt_request_id: gui_message.opt_request_id,
                variant,
            };
            match serde_json::to_string(&msg) {
                Ok(text) => HandleGuiMessageM::Ok(Some(text)),
                Err(e) => {
                    p!(msg);
                    p!(e);
                    todo!()
                }
            }
        } else {
            HandleGuiMessageM::Ok(None)
        }
    }
}

impl HuskyDebuggerInternal {
    #[cfg(feature = "verify_consistency")]
    fn handle_gui_message(
        &mut self,
        request: &HuskyTracerGuiMessage,
    ) -> Option<HuskyTracerServerMessageVariant> {
        if let Some(request_id) = request.opt_request_id {
            if self.next_request_id != request_id {
                // make sure all requests are received in order
                match request.variant {
                    HuskyTracerGuiMessageVariant::HotReloadRequest => {
                        self.next_request_id = request_id + 1;
                    }
                    _ => {
                        p!(request, self.next_request_id, request_id);
                        panic!("todo: replace panic with caching or warning")
                    }
                }
            } else {
                self.next_request_id += 1
            }
        }
        match request.variant {
            HuskyTracerGuiMessageVariant::HotReloadRequest => {
                Some(HuskyTracerServerMessageVariant::HotReload {
                    init_data: self.tracetime.init_data(),
                })
            }
            HuskyTracerGuiMessageVariant::Activate {
                trace_id,
                needs_figure_canvases,
                needs_figure_controls,
            } => self.handle_activate(
                trace_id,
                needs_figure_canvases,
                needs_figure_controls,
                request,
            ),
            HuskyTracerGuiMessageVariant::ToggleExpansion { trace_id } => {
                if let Some((new_traces, subtrace_ids, trace_stalks, trace_stats)) =
                    self.tracetime.toggle_expansion(trace_id)
                {
                    Some(HuskyTracerServerMessageVariant::ToggleExpansion {
                        new_traces,
                        subtrace_ids,
                        trace_stalks,
                        trace_stats,
                    })
                } else {
                    // ad hoc; should panic here
                    if request.opt_request_id.is_some() {
                        Some(HuskyTracerServerMessageVariant::ToggleExpansion {
                            new_traces: Default::default(),
                            subtrace_ids: Default::default(),
                            trace_stalks: Default::default(),
                            trace_stats: Default::default(),
                        })
                    } else {
                        None
                    }
                }
            }
            HuskyTracerGuiMessageVariant::ToggleShow { trace_id } => {
                self.tracetime.toggle_show(trace_id);
                None
            }
            HuskyTracerGuiMessageVariant::Trace { id } => {
                let trace = self.tracetime.trace(id);
                Some(HuskyTracerServerMessageVariant::Trace {
                    trace_props: trace.raw_data.clone(),
                })
            }
            HuskyTracerGuiMessageVariant::TraceStalk { trace_id } => {
                let (_, stalk) = self.tracetime.keyed_trace_stalk(trace_id);
                Some(HuskyTracerServerMessageVariant::TraceStalk { stalk })
            }
            HuskyTracerGuiMessageVariant::SetRestriction {
                ref restriction,
                needs_figure_canvases,
                needs_figure_controls,
                ref new_stalk_keys,
                ref new_stats_keys,
            } => self.handle_set_restriction(
                restriction,
                needs_figure_canvases,
                needs_figure_controls,
                new_stalk_keys,
                new_stats_keys,
            ),
            HuskyTracerGuiMessageVariant::UpdateFigureControlData {
                trace_id,
                ref restriction,
                ref figure_control_props,
            } => {
                self.tracetime.update_figure_control(
                    trace_id,
                    restriction,
                    figure_control_props.clone(),
                );
                None
            }
            HuskyTracerGuiMessageVariant::TogglePin {
                trace_id,
                needs_figure_canvases,
                needs_figure_controls,
            } => self.handle_toggle_pin(
                trace_id,
                needs_figure_canvases,
                needs_figure_controls,
                request,
            ),
        }
    }

    #[cfg(not(feature = "verify_consistency"))]
    fn handle_gui_message(
        &mut self,
        request: &HuskyTracerGuiMessage,
    ) -> HandleGuiMessageM<Option<HuskyTracerServerMessageVariant>> {
        use husky_vm::__VMErrorVariant;

        if let Some(request_id) = request.opt_request_id {
            if self.next_request_id != request_id {
                // make sure all requests are received in order
                match request.variant {
                    HuskyTracerGuiMessageVariant::HotReloadRequest => {
                        self.next_request_id = request_id + 1;
                    }
                    _ => {
                        p!(request, self.next_request_id, request_id);
                        panic!("todo: replace panic with caching or warning")
                    }
                }
            } else {
                self.next_request_id += 1
            }
        }
        HandleGuiMessageM::Ok(match request.variant {
            HuskyTracerGuiMessageVariant::HotReloadRequest => {
                Some(HuskyTracerServerMessageVariant::HotReload {
                    init_data: self.hot_reload()?,
                })
            }
            HuskyTracerGuiMessageVariant::Activate {
                trace_id,
                needs_figure_canvases,
                needs_figure_controls,
            } => self.handle_activate(
                trace_id,
                needs_figure_canvases,
                needs_figure_controls,
                request,
            ),
            HuskyTracerGuiMessageVariant::ToggleExpansion { trace_id } => {
                let opt_results = match self.tracetime.toggle_expansion(trace_id) {
                    Ok(opt_results) => opt_results,
                    Err(e) => {
                        match e.variant() {
                            __VMErrorVariant::Normal => todo!(),
                            __VMErrorVariant::FromBatch { sample_id } => {
                                assert!(
                                    self.tracetime.restriction().is_generic()
                                        || self.tracetime.restriction().sample_id()
                                            != SampleId(*sample_id)
                                );
                                p!(
                                    self.tracetime.restriction().sample_id(),
                                    SampleId(*sample_id)
                                );
                                todo!()
                            }
                        }
                        todo!()
                    }
                };
                if let Some((new_traces, subtrace_ids, trace_stalks, trace_stats)) = opt_results {
                    Some(HuskyTracerServerMessageVariant::ToggleExpansion {
                        new_traces,
                        subtrace_ids,
                        trace_stalks,
                        trace_stats,
                    })
                } else {
                    // ad hoc; should panic here
                    if request.opt_request_id.is_some() {
                        Some(HuskyTracerServerMessageVariant::ToggleExpansion {
                            new_traces: Default::default(),
                            subtrace_ids: Default::default(),
                            trace_stalks: Default::default(),
                            trace_stats: Default::default(),
                        })
                    } else {
                        None
                    }
                }
            }
            HuskyTracerGuiMessageVariant::ToggleShow { trace_id } => {
                self.tracetime.toggle_show(trace_id);
                None
            }
            HuskyTracerGuiMessageVariant::Trace { id } => {
                let trace = self.tracetime.trace(id);
                Some(HuskyTracerServerMessageVariant::Trace {
                    trace_props: trace.raw_data.clone(),
                })
            }
            HuskyTracerGuiMessageVariant::TraceStalk { trace_id } => {
                let (_, stalk) = self.tracetime.keyed_trace_stalk(trace_id);
                Some(HuskyTracerServerMessageVariant::TraceStalk { stalk })
            }
            HuskyTracerGuiMessageVariant::SetRestriction {
                ref restriction,
                needs_figure_canvases,
                needs_figure_controls,
                needs_stalks,
                needs_statss,
            } => self.handle_set_restriction(
                restriction,
                needs_figure_canvases,
                needs_figure_controls,
                needs_stalks,
                needs_statss,
            ),
            HuskyTracerGuiMessageVariant::UpdateFigureControlData {
                trace_id,
                ref figure_control_data,
            } => {
                self.tracetime
                    .set_figure_control(trace_id, figure_control_data.clone());
                None
            }
            HuskyTracerGuiMessageVariant::TogglePin {
                trace_id,
                needs_figure_canvases,
                needs_figure_controls,
            } => self.handle_toggle_pin(
                trace_id,
                needs_figure_canvases,
                needs_figure_controls,
                request,
            ),
        })
    }

    fn handle_activate(
        &mut self,
        trace_id: TraceId,
        needs_figure_canvases: bool,
        needs_figure_controls: bool,
        request: &HuskyTracerGuiMessage,
    ) -> Option<HuskyTracerServerMessageVariant> {
        match self.tracetime.activate(trace_id) {
            Ok((new_figure_canvases, new_figure_controls)) => {
                let needs_response = needs_figure_canvases || needs_figure_controls;
                should_eq!(request.opt_request_id.is_some(), needs_response);
                if needs_response {
                    Some(HuskyTracerServerMessageVariant::Activate {
                        new_figure_canvases,
                        new_figure_controls,
                    })
                } else {
                    None
                }
            }
            Err(e) => {
                p!(e);
                todo!()
            }
        }
    }

    fn handle_toggle_pin(
        &mut self,
        trace_id: TraceId,
        needs_figure_canvases: bool,
        needs_figure_controls: bool,
        request: &HuskyTracerGuiMessage,
    ) -> Option<HuskyTracerServerMessageVariant> {
        match self.tracetime.toggle_pin(trace_id) {
            Ok((new_figure_canvases, new_figure_controls)) => {
                let needs_response = needs_figure_canvases || needs_figure_controls;
                should_eq!(request.opt_request_id.is_some(), needs_response);
                if needs_response {
                    Some(HuskyTracerServerMessageVariant::TogglePin {
                        new_figure_canvases,
                        new_figure_controls,
                    })
                } else {
                    None
                }
            }
            Err(_) => todo!(),
        }
    }

    #[cfg(feature = "verify_consistency")]
    fn handle_set_restriction(
        &mut self,
        restriction: &Restriction,
        needs_figure_canvases: bool,
        needs_figure_controls: bool,
        new_stalk_keys: &[TraceStalkKey],
        new_stats_keys: &[TraceStatsKey],
    ) -> Option<HuskyTracerServerMessageVariant> {
        let (new_trace_stalks, new_trace_stats) =
            self.tracetime.set_restriction(restriction.clone());
        if needs_figure_canvases
            || needs_figure_controls
            || new_stalk_keys.len() > 0
            || new_stats_keys.len() > 0
        {
            let (opt_figure_canvas_data, opt_figure_control_data) = if let Some(active_trace_id) =
                self.tracetime.opt_active_trace_id()
            {
                let opt_figure_canvas_data = if needs_figure_canvases {
                    match self.tracetime.figure_canvas(active_trace_id) {
                        Ok(figure_canvas) => Some(figure_canvas),
                        Err((sample_id, error)) => {
                            return Some(HuskyTracerServerMessageVariant::SetRestrictionWithError {
                                sample_id,
                                error: format!("{:?}", error),
                            })
                        }
                    }
                } else {
                    None
                };
                let opt_figure_control_data = if needs_figure_controls {
                    Some(self.tracetime.figure_control(active_trace_id))
                } else {
                    None
                };
                (opt_figure_canvas_data, opt_figure_control_data)
            } else {
                (None, None)
            };
            if new_stalk_keys.len() > 0 {
                assert!(new_trace_stalks.len() > 0);
            }
            Some(HuskyTracerServerMessageVariant::SetRestriction {
                opt_figure_canvas_data,
                opt_figure_control_data,
                new_trace_stalks,
                new_trace_stats,
            })
        } else {
            None
        }
    }

    #[cfg(not(feature = "verify_consistency"))]
    fn handle_set_restriction(
        &mut self,
        restriction: &Restriction,
        needs_figure_canvases: bool,
        needs_figure_controls: bool,
        needs_stalks: bool,
        needs_statss: bool,
    ) -> Option<HuskyTracerServerMessageVariant> {
        match self.tracetime.set_restriction(restriction.clone()) {
            Ok((new_figure_canvases, new_figure_controls, new_trace_stalks, new_trace_statss)) => {
                assert_eq!(needs_figure_canvases, new_figure_canvases.len() > 0);
                assert_eq!(needs_figure_controls, new_figure_controls.len() > 0);
                assert_eq!(needs_stalks, new_trace_stalks.len() > 0);
                assert_eq!(needs_statss, new_trace_statss.len() > 0);
                if needs_figure_canvases || needs_figure_controls || needs_stalks || needs_statss {
                    Some(HuskyTracerServerMessageVariant::SetRestriction {
                        new_figure_canvases,
                        new_figure_controls,
                        new_trace_stalks,
                        new_trace_statss,
                    })
                } else {
                    None
                }
            }
            Err(_) => todo!(),
        }
    }
}
