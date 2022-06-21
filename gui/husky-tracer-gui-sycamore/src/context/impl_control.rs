mod attention;

use super::*;
use web_sys::{Event, KeyboardEvent};

impl DebuggerContext {
    pub fn toggle_expansion_handler(&self, trace_id: TraceId) -> Rc<dyn Fn()> {
        let this = self.clone();
        Rc::new(move || this.toggle_expansion(trace_id))
    }

    pub fn activate_handler(&self, trace_id: TraceId) -> impl Fn(Event) {
        let this = self.clone();
        move |_| this.activate(trace_id)
    }

    pub fn toggle_attention_kind_handler(&self) -> impl Fn(Event) {
        let this = self.clone();
        move |_| this.toggle_attention_kind()
    }

    pub fn set_attention_from_dialog_handler(&self) -> impl Fn() {
        let this = self.clone();
        move || this.set_attention_from_dialog()
    }

    pub fn keydown_handler(&self) -> impl Fn(Event) {
        let this = self.clone();
        move |ev| {
            if !this.dialog_opened.cget() {
                let ev: KeyboardEvent = ev.unchecked_into();
                let c = char::from_u32(ev.key_code()).unwrap();
                match c {
                    'T' => {
                        // 't'
                        todo!("t")
                    }
                    'C' => {
                        // 't'
                        log::info!("figure context is \n:{:?}", this.figure_context);
                        // log::info!("fcous context is \n:{:?}", this.attention_context);
                        log::info!(
                            "opt active trace id is \n:{:?}",
                            this.trace_context.opt_active_trace_id
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

    fn activate(&self, trace_id: TraceId) {
        let attention = self.attention_context.attention.get();
        let trace = self.trace_context.trace(trace_id);
        let is_figure_cached = self.figure_context.is_figure_cached(&trace, &attention);
        if (is_figure_cached) {
            self.trace_context.did_activate(trace_id);
            self.ws.send_message(
                HuskyTracerGuiMessageVariant::Activate {
                    trace_id,
                    opt_attention_for_figure: None,
                },
                None,
            );
        } else {
            let this = self.clone();
            self.ws.send_message(
                HuskyTracerGuiMessageVariant::Activate {
                    trace_id,
                    opt_attention_for_figure: Some((*attention).clone()),
                },
                Some(Box::new(move |message| match message.variant {
                    HuskyTracerServerMessageVariant::Activate {
                        figure_canvas_data,
                        figure_control_data,
                    } => {
                        this.figure_context.set_figure(
                            &trace,
                            &attention,
                            figure_canvas_data,
                            figure_control_data,
                        );
                        this.trace_context.did_activate(trace_id);
                    }
                    _ => panic!(),
                })),
            );
        }
    }

    fn toggle_expansion(&self, trace_id: TraceId) {
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
                let this = self.clone();
                self.ws.send_message(
                    HuskyTracerGuiMessageVariant::ToggleExpansion { trace_id },
                    Some(Box::new(move |message| match message.variant {
                        HuskyTracerServerMessageVariant::ToggleExpansion {
                            new_traces,
                            subtrace_ids,
                            trace_stalks,
                        } => {
                            this.trace_context.receive_subtraces(key, subtrace_ids);
                            this.trace_context.receive_new_traces(new_traces);
                            this.trace_context.receive_trace_stalks(trace_stalks);
                            expansion.set(true)
                        }
                        _ => panic!(),
                    })),
                )
            }
        }
    }
}
