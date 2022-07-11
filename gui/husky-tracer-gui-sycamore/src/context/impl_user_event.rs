mod arrival;
mod attention;
mod enter;
mod expansion;
mod pin;
mod shown;
mod utils;

use super::*;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, KeyboardEvent};

enum UserEvent {
    ToggleExpansion { trace_id: TraceId },
    ToggleArrival { trace_id: TraceId },
    TogglePin { trace_id: TraceId },
    ToggleEnter { trace_id: TraceId },
    ToggleShown { trace_id: TraceId },
    Activate { trace_id: TraceId },
    SetAttention { attention: Attention },
}

impl UserEvent {
    fn set_attention_from_dialog() -> Self {
        let sample_id_value = get_element_by_id::<HtmlInputElement>("sample-id-input").value();
        loop {
            match sample_id_value.parse::<usize>() {
                Ok(raw) => {
                    let attention_dialog =
                        get_element_by_id::<HtmlDialogElement>("attention-dialog");
                    attention_dialog.close();
                    break UserEvent::SetAttention {
                        attention: Attention::Specific {
                            sample_id: SampleId(raw),
                        },
                    };
                }
                Err(_) => alert!("`{}` is not a valid sample id", sample_id_value),
            }
        }
    }

    fn toggle_attention_kind(ctx: &'static DebuggerContext) -> Self {
        UserEvent::SetAttention {
            attention: ctx.attention_context.toggled_attention_kind(),
        }
    }

    fn keydown(ctx: &'static DebuggerContext, ev: Event) -> Option<Self> {
        if !ctx.dialog_opened.cget() {
            let ev: KeyboardEvent = ev.unchecked_into();
            let c = char::from_u32(ev.key_code()).unwrap();
            match c {
                'T' => {
                    // 't'
                    log::info!("active trace is {:?}", ctx.trace_context.opt_active_trace())
                }
                'C' => {
                    // 't'
                    log::info!("figure context is \n:{:?}", ctx.figure_context);
                    // log::info!("fcous context is \n:{:?}", self.attention_context);
                    log::info!(
                        "opt active trace id is \n:{:?}",
                        ctx.trace_context.opt_active_trace_id
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
        None
    }
}

impl DebuggerContext {
    fn handle_user_event(&'static self, event: UserEvent) {
        // todo: record user events
        match event {
            UserEvent::ToggleExpansion { trace_id } => self.toggle_expansion(trace_id),
            UserEvent::Activate { trace_id } => self.activate(trace_id),
            UserEvent::SetAttention { attention } => self.set_attention(attention),
            UserEvent::ToggleArrival { trace_id } => self.toggle_arrival(trace_id),
            UserEvent::TogglePin { trace_id } => self.toggle_pin(trace_id),
            UserEvent::ToggleEnter { trace_id } => self.toggle_enter(trace_id),
            UserEvent::ToggleShown { trace_id } => self.toggle_shown(trace_id),
        }
    }

    pub fn toggle_expansion_handler(&'static self, trace_id: TraceId) -> Rc<dyn Fn()> {
        Rc::new(move || self.handle_user_event(UserEvent::ToggleExpansion { trace_id }))
    }

    pub fn toggle_shown_handler(&'static self, trace_id: TraceId) -> impl Fn() {
        move || self.handle_user_event(UserEvent::ToggleShown { trace_id })
    }

    pub fn toggle_arrival_handler(&'static self, trace_id: TraceId) -> impl Fn() {
        move || self.handle_user_event(UserEvent::ToggleArrival { trace_id })
    }

    pub fn toggle_pin_handler(&'static self, trace_id: TraceId) -> impl Fn() {
        move || self.handle_user_event(UserEvent::TogglePin { trace_id })
    }

    pub fn toggle_enter_handler(&'static self, trace_id: TraceId) -> impl Fn() {
        move || self.handle_user_event(UserEvent::ToggleEnter { trace_id })
    }

    pub fn activate_handler(&'static self, trace_id: TraceId) -> impl Fn(Event) {
        move |_| self.handle_user_event(UserEvent::Activate { trace_id })
    }

    pub fn toggle_attention_kind_handler(&'static self) -> impl Fn(Event) {
        move |_| self.handle_user_event(UserEvent::toggle_attention_kind(self))
    }

    pub fn set_attention_from_dialog_handler(&'static self) -> impl Fn() {
        move || self.handle_user_event(UserEvent::set_attention_from_dialog())
    }

    pub fn keydown_handler(&'static self) -> impl Fn(Event) {
        move |ev| {
            if let Some(event) = UserEvent::keydown(self, ev) {
                self.handle_user_event(event)
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
}
