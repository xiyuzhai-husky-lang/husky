mod activate;
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

    pub fn set_restriction_from_dialog_handler(&'static self) -> impl Fn(Event) {
        move |_| {
            let dialog = restriction_dialog();
            sample_id_input().set_value("");
            add_event_listener!(dialog, "keydown", move |event: web_sys::UiEvent| {
                let event: KeyboardEvent = event.unchecked_into();
                match event.key().as_str() {
                    "Enter" => {
                        let sample_id_value = sample_id_input().value();
                        match sample_id_value.parse::<usize>() {
                            Ok(raw) => {
                                restriction_dialog().close();
                                self.handle_status_change(StatusChange::update_restriction(
                                    self,
                                    |res| res.set_sample_id(SampleId(raw)),
                                ))
                            }
                            Err(_) => alert!("`{}` is not a valid sample id", sample_id_value),
                        }
                    }
                    _ => (),
                }
            });
            dialog.show_modal();
        }
    }

    pub fn keydown_handler(&'static self) -> impl Fn(Event) {
        move |ev| {
            if let Some(event) = StatusChange::keydown(self, ev) {
                self.handle_status_change(event)
            }
        }
    }

    pub fn add_partition_handler(&'static self, idx: usize) -> impl Fn(Event) {
        move |_| {
            partition_ncol_input().set_value("2");
            log::info!("add_partition_handler called");
            let dialog = new_partition_dialog();
            assert!(!dialog.open());
            dialog.show_modal();
            add_event_listener!(dialog, "keydown", {
                move |event: web_sys::UiEvent| {
                    event.stop_propagation();
                    let event: KeyboardEvent = event.unchecked_into();
                    match event.key().as_str() {
                        "Enter" => {
                            let label_str = partition_input().value();
                            let label_raw = match label_str.parse::<usize>() {
                                Ok(raw) => raw,
                                Err(_) => {
                                    alert!("`{}` is not a valid partition", label_str);
                                    return;
                                }
                            };
                            let ncol_str = partition_ncol_input().value();
                            let ncol = match ncol_str.parse::<u32>() {
                                Ok(raw) => raw,
                                Err(_) => {
                                    alert!("`{}` is not a valid partition", label_str);
                                    return;
                                }
                            };
                            let new_partition = PartitionDefnData {
                                ncol,
                                variant: PartitionDefnDataVariant::Label(Label(label_raw as i32)),
                            };
                            new_partition_dialog().close();
                            self.handle_status_change(StatusChange::update_restriction(
                                self,
                                |res| res.add_partition(idx, new_partition),
                            ))
                        }
                        _ => (),
                    }
                }
            })
        }
    }
}
