use super::*;
use check_utils::should_eq;
use colored::Colorize;
use std::panic::{catch_unwind, panic_any};
use std::{collections::hash_map::DefaultHasher, hash::Hasher};
use std::{hash::Hash, path::PathBuf};
use wild_utils::ref_to_mut_ref;

pub fn handle_message(
    debugger: Arc<HuskyDebugger>,
    client_sender: UnboundedSender<Result<Message, warp::Error>>,
    gui_messages: &[HuskyTracerGuiMessage],
) {
    let debugger_ = debugger.clone();
    let client_sender_ = client_sender.clone();
    let latest_gui_message = gui_messages.last().unwrap();
    match catch_unwind(|| debugger_.handle_gui_message(&latest_gui_message)) {
        Ok(Some(text)) => match client_sender_.send(Ok(Message::text(text))) {
            Ok(_) => (),
            Err(_) => todo!(),
        },
        Ok(None) => (),
        Err(e) => save_server_history(gui_messages, e),
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct DebuggerServerHistory {
    gui_messages: Vec<HuskyTracerGuiMessage>,
}

fn save_server_history(
    gui_messages: &[HuskyTracerGuiMessage],
    e: Box<dyn std::any::Any + std::marker::Send>,
) {
    let value = serde_json::to_string(gui_messages).unwrap();
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let filename = format!("gui-messages-with-hash-{}.json", hasher.finish());
    let filename: &str = &filename;
    let husky_dir: PathBuf = std::env::var("HUSKY_DIR").unwrap().into();
    let husky_dir: PathBuf = std::env::var("HUSKY_DIR").unwrap().into();
    let filepath: PathBuf = husky_dir.join(format!("test-examples/debugger/server/{filename}"));
    io_utils::diff_write(&filepath, &value);
    println!("{}", "server history saved".red())
}

impl HuskyDebugger {
    fn handle_gui_message(self: Arc<Self>, gui_message: &HuskyTracerGuiMessage) -> Option<String> {
        let internal: &mut HuskyDebuggerInternal = &mut self.internal.lock().unwrap();
        let opt_response_variant = internal.handle_gui_message(gui_message);
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

impl HuskyDebuggerInternal {
    fn handle_gui_message(
        &mut self,
        request: &HuskyTracerGuiMessage,
    ) -> Option<HuskyTracerServerMessageVariant> {
        p!(request);
        if let Some(request_id) = request.opt_request_id {
            if self.next_request_id != request_id {
                // make sure all requests are received in order
                match request.variant {
                    HuskyTracerGuiMessageVariant::InitDataRequest => {
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
            HuskyTracerGuiMessageVariant::InitDataRequest => {
                Some(HuskyTracerServerMessageVariant::Init {
                    init_data: self.trace_time.init_data(),
                })
            }
            HuskyTracerGuiMessageVariant::Activate {
                trace_id,
                needs_figure_canvas_data,
                needs_figure_control_data,
            } => {
                self.trace_time.activate(trace_id);
                let needs_response = needs_figure_canvas_data || needs_figure_control_data;
                should_eq!(request.opt_request_id.is_some(), needs_response);
                if needs_response {
                    let opt_figure_canvas_data = if needs_figure_canvas_data {
                        Some(match self.trace_time.figure_canvas(trace_id) {
                            Ok(figure_canvas_data) => figure_canvas_data,
                            Err((sample_id0, error)) => {
                                match self.trace_time.attention() {
                                    Attention::Specific { sample_id } => {
                                        if *sample_id != sample_id0 {
                                            return Some(
                                            HuskyTracerServerMessageVariant::ActivateWithError {
                                                sample_id: sample_id0,
                                                error: format!("{:?}", error),
                                            },
                                        );
                                        }
                                    }
                                    Attention::Generic {
                                        partitions,
                                        constraints,
                                    } => (),
                                }
                                FigureCanvasData::EvalError {
                                    message: format!("{:?}", error),
                                }
                            }
                        })
                    } else {
                        None
                    };
                    let opt_figure_control_data = if needs_figure_control_data {
                        Some(self.trace_time.figure_control(trace_id))
                    } else {
                        None
                    };
                    Some(HuskyTracerServerMessageVariant::Activate {
                        opt_figure_canvas_data,
                        opt_figure_control_data,
                    })
                } else {
                    None
                }
            }
            HuskyTracerGuiMessageVariant::ToggleExpansion { trace_id } => {
                if let Some((new_traces, subtrace_ids, trace_stalks)) =
                    self.trace_time.toggle_expansion(trace_id)
                {
                    Some(HuskyTracerServerMessageVariant::ToggleExpansion {
                        new_traces,
                        subtrace_ids,
                        trace_stalks,
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
                    trace_props: trace.raw_data.clone(),
                })
            }
            HuskyTracerGuiMessageVariant::TraceStalk { trace_id } => {
                let (_, stalk) = self.trace_time.keyed_trace_stalk(trace_id);
                Some(HuskyTracerServerMessageVariant::TraceStalk { stalk })
            }
            HuskyTracerGuiMessageVariant::LockAttention {
                ref attention,
                needs_figure_canvas_data,
                needs_figure_control_data,
                needs_stalk,
            } => {
                let new_trace_stalks = self.trace_time.set_attention(attention.clone());
                if needs_figure_canvas_data || needs_figure_control_data || needs_stalk {
                    let (opt_figure_canvas_data, opt_figure_control_data) =
                        if let Some(active_trace_id) = self.trace_time.opt_active_trace_id() {
                            let opt_figure_canvas_data = if needs_figure_canvas_data {
                                match self.trace_time.figure_canvas(active_trace_id) {
                                    Ok(figure_canvas) => Some(figure_canvas),
                                    Err((sample_id, error)) => return Some(
                                        HuskyTracerServerMessageVariant::LockAttentionWithError {
                                            sample_id,
                                            error: format!("{:?}", error),
                                        },
                                    ),
                                }
                            } else {
                                None
                            };
                            let opt_figure_control_data = if needs_figure_control_data {
                                Some(self.trace_time.figure_control(active_trace_id))
                            } else {
                                None
                            };
                            (opt_figure_canvas_data, opt_figure_control_data)
                        } else {
                            (None, None)
                        };
                    p!(self.trace_time.trace_stalks);
                    if needs_stalk {
                        assert!(new_trace_stalks.len() > 0);
                    }
                    Some(HuskyTracerServerMessageVariant::LockAttention {
                        opt_figure_canvas_data,
                        opt_figure_control_data,
                        new_trace_stalks,
                    })
                } else {
                    None
                }
            }
            HuskyTracerGuiMessageVariant::UpdateFigureControlData {
                trace_id,
                ref attention,
                ref figure_control_props,
            } => {
                self.trace_time.update_figure_control(
                    trace_id,
                    attention,
                    figure_control_props.clone(),
                );
                None
            }
        }
    }
}
