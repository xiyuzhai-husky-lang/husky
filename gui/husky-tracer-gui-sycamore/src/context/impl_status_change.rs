mod expansion;
mod pin;
mod restriction;
mod shown;
mod utils;

use super::*;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, KeyboardEvent};

enum StatusChange {
    ToggleExpansion { trace_id: TraceId },
    TogglePin { trace_id: TraceId },
    ToggleShown { trace_id: TraceId },
    Activate { trace_id: TraceId },
    SetRestriction { restriction: Restriction },
}

impl StatusChange {
    fn set_restriction_from_dialog(ctx: &'static DebuggerContext) -> Self {
        let sample_id_value = get_element_by_id::<HtmlInputElement>("sample-id-input").value();
        loop {
            match sample_id_value.parse::<usize>() {
                Ok(raw) => {
                    let restriction_dialog =
                        get_element_by_id::<HtmlDialogElement>("restriction-dialog");
                    restriction_dialog.close();
                    // let mut restriction = ctx.restriction_context.restriction.cget();
                    // restriction.specific_sample_id = SampleId(raw);
                    break StatusChange::update_restriction(ctx, |res| {
                        res.set_sample_id(SampleId(raw))
                    });
                }
                Err(_) => alert!("`{}` is not a valid sample id", sample_id_value),
            }
        }
    }

    fn update_restriction(
        ctx: &'static DebuggerContext,
        update: impl FnOnce(&mut Restriction),
    ) -> Self {
        let mut restriction = ctx.restriction_context.restriction.cget();
        update(&mut restriction);
        StatusChange::SetRestriction { restriction }
    }

    fn toggle_restriction_is_specific(ctx: &'static DebuggerContext) -> Self {
        Self::update_restriction(ctx, |res| res.toggle_is_specific())
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
                    // log::info!("fcous context is \n:{:?}", self.restriction_context);
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
    fn handle_status_change(&'static self, event: StatusChange) {
        // todo: record user events
        match event {
            StatusChange::ToggleExpansion { trace_id } => self.toggle_expansion(trace_id),
            StatusChange::ToggleShown { trace_id } => self.toggle_shown(trace_id),
            StatusChange::Activate { trace_id } => self.activate(trace_id),
            StatusChange::SetRestriction { restriction } => self.set_restriction(restriction),
            // self.set_restriction(restriction),
            // StatusChange::ToggleArrival { trace_id } => {
            //     self.update_restriction(|res| res.toggle_arrival(trace_id))
            // }
            // StatusChange::ToggleEnter { trace_id } => {
            //     self.update_restriction(|res| res.toggle_enter(trace_id))
            // }
            StatusChange::TogglePin { trace_id } => self.toggle_pin(trace_id),
        }
    }

    pub fn toggle_expansion_handler(&'static self, trace_id: TraceId) -> Rc<dyn Fn()> {
        Rc::new(move || self.handle_status_change(StatusChange::ToggleExpansion { trace_id }))
    }

    pub fn toggle_shown_handler(&'static self, trace_id: TraceId) -> impl Fn() {
        move || self.handle_status_change(StatusChange::ToggleShown { trace_id })
    }

    pub fn toggle_arrival_handler(&'static self, trace_id: TraceId) -> impl Fn() {
        move || {
            self.handle_status_change(StatusChange::update_restriction(self, |res| {
                res.toggle_arrival(trace_id)
            }))
        }
    }

    pub fn toggle_pin_handler(&'static self, trace_id: TraceId) -> impl Fn() {
        move || self.handle_status_change(StatusChange::TogglePin { trace_id })
    }

    pub fn toggle_enter_handler(&'static self, trace_id: TraceId) -> impl Fn() {
        move || {
            self.handle_status_change(StatusChange::update_restriction(self, |res| {
                res.toggle_enter(trace_id)
            }))
        }
    }

    pub fn activate_handler(&'static self, trace_id: TraceId) -> impl Fn(Event) {
        move |_| self.handle_status_change(StatusChange::Activate { trace_id })
    }

    pub fn toggle_restriction_kind_handler(&'static self) -> impl Fn(Event) {
        move |_| self.handle_status_change(StatusChange::toggle_restriction_is_specific(self))
    }

    pub fn set_restriction_from_dialog_handler(&'static self) -> impl Fn() {
        move || self.handle_status_change(StatusChange::set_restriction_from_dialog(self))
    }

    pub fn keydown_handler(&'static self) -> impl Fn(Event) {
        move |ev| {
            if let Some(event) = StatusChange::keydown(self, ev) {
                self.handle_status_change(event)
            }
        }
    }

    fn activate(&'static self, new_active_trace_id: TraceId) {
        let restriction = self.restriction_context.restriction.get();
        let trace = self.trace_context.trace_data(new_active_trace_id);
        let needs_figure_canvas_data =
            self.needs_figure_canvas_data(Some(new_active_trace_id), &restriction);
        let needs_figure_control_data =
            self.needs_figure_control_data(Some(new_active_trace_id), &restriction);
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
                            &restriction,
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
