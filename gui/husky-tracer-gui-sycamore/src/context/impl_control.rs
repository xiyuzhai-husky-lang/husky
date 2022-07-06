mod attention;
mod shown;
mod utils;

use super::*;
use web_sys::{Event, KeyboardEvent};

impl DebuggerContext {
    pub fn toggle_expansion_handler(&'static self, trace_id: TraceId) -> Rc<dyn Fn()> {
        Rc::new(move || self.toggle_expansion(trace_id))
    }

    pub fn activate_handler(&'static self, trace_id: TraceId) -> impl Fn(Event) {
        move |_| self.activate(trace_id)
    }

    pub fn toggle_attention_kind_handler(&'static self) -> impl Fn(Event) {
        move |_| self.toggle_attention_kind()
    }

    pub fn set_attention_from_dialog_handler(&'static self) -> impl Fn() {
        move || self.set_attention_from_dialog()
    }

    pub fn keydown_handler(&'static self) -> impl Fn(Event) {
        move |ev| {
            if !self.dialog_opened.cget() {
                let ev: KeyboardEvent = ev.unchecked_into();
                let c = char::from_u32(ev.key_code()).unwrap();
                match c {
                    'T' => {
                        // 't'
                        log::info!(
                            "active trace is {:?}",
                            self.trace_context.opt_active_trace()
                        )
                    }
                    'C' => {
                        // 't'
                        log::info!("figure context is \n:{:?}", self.figure_context);
                        // log::info!("fcous context is \n:{:?}", self.attention_context);
                        log::info!(
                            "opt active trace id is \n:{:?}",
                            self.trace_context.opt_active_trace_id
                        );
                    }
                    'J' => {
                        todo!()
                    }
                    'K' => {
                        todo!()
                    }
                    'L' => {
                        todo!()
                    }
                    'H' => {
                        todo!()
                    }
                    _ => log::info!("keydown with char: {}", c),
                }
            }
        }
    }

    fn activate(&'static self, new_active_trace_id: TraceId) {
        let attention = self.attention_context.attention.get();
        let trace = self.trace_context.trace_data(new_active_trace_id);
        let needs_figure_canvas_data =
            self.needs_figure_canvas_data(Some(new_active_trace_id), &attention);
        let needs_figure_control_data =
            self.needs_figure_control_data(Some(new_active_trace_id), &attention);
        let needs_response = needs_figure_control_data || needs_figure_control_data;
        self.ws.send_message(
            HuskyTracerGuiMessageVariant::Activate {
                trace_id: new_active_trace_id,
                needs_figure_canvas_data,
                needs_figure_control_data,
            },
            if needs_response {
                Some(Box::new(move |response| match response.variant {
                    HuskyTracerServerMessageVariant::Activate {
                        opt_figure_canvas_data,
                        opt_figure_control_data,
                    } => {
                        self.figure_context.set_opt_figure_data(
                            self.scope,
                            &trace,
                            &attention,
                            opt_figure_canvas_data.map(|data| self.alloc_value(data)),
                            opt_figure_control_data,
                        );
                        self.trace_context.did_activate(new_active_trace_id);
                    }
                    HuskyTracerServerMessageVariant::ActivateWithError { .. } => todo!(),
                    _ => panic!("unexpected response {:?}", response),
                }))
            } else {
                {
                    self.trace_context.did_activate(new_active_trace_id);
                    None
                }
            },
        );
    }

    fn toggle_expansion(&'static self, trace_id: TraceId) {
        let expansion = self.trace_context.expanded_signal(trace_id);
        if expansion.cget() {
            expansion.set(false)
        } else {
            let attention = self.attention_context.attention.get();
            let trace_kind = self.trace_context.trace_kind(trace_id);
            let key = SubtracesKey::new(&attention, trace_kind, trace_id);
            if self
                .trace_context
                .subtrace_ids_map
                .borrow(file!(), line!())
                .contains_key(&key)
            {
                self.ws.send_message(
                    HuskyTracerGuiMessageVariant::ToggleExpansion { trace_id },
                    None,
                );
                expansion.set(true)
            } else {
                self.ws.send_message(
                    HuskyTracerGuiMessageVariant::ToggleExpansion { trace_id },
                    Some(Box::new(move |message| match message.variant {
                        HuskyTracerServerMessageVariant::ToggleExpansion {
                            new_traces,
                            subtrace_ids,
                            trace_stalks,
                        } => {
                            self.trace_context
                                .receive_subtraces(key, self.alloc_value(subtrace_ids));
                            self.trace_context.receive_traces(
                                new_traces
                                    .into_iter()
                                    .map(|trace| TraceNodeState::from_data(self.scope, trace)),
                            );
                            self.trace_context.receive_trace_stalks(
                                trace_stalks
                                    .into_iter()
                                    .map(|(k, v)| (k, self.alloc_value(v))),
                            );
                            expansion.set(true)
                        }
                        _ => panic!(),
                    })),
                )
            }
        }
    }
}
