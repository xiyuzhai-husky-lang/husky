mod activate;
mod expansion;
mod pin;
mod restriction;
mod shown;
mod utils;

use super::*;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, KeyboardEvent};

pub(super) enum StatusChange {
    ToggleExpansion { trace_id: TraceId },
    TogglePin { trace_id: TraceId },
    ToggleShown { trace_id: TraceId },
    Activate { trace_id: TraceId },
    SetPresentation { presentation: Presentation },
}

impl StatusChange {
    pub(super) fn update_restriction(
        ctx: &'static DeveloperGuiContext,
        update: impl FnOnce(&mut Presentation),
    ) -> Self {
        let mut presentation = ctx.presentation_signal.cget();
        update(&mut presentation);
        StatusChange::SetPresentation { presentation }
    }

    fn toggle_restriction_is_specific(ctx: &'static DeveloperGuiContext) -> Self {
        Self::update_restriction(ctx, |res| res.toggle_kind())
    }

    fn keydown(ctx: &'static DeveloperGuiContext, ev: Event) -> Option<Self> {
        if !ctx.dialog_opened.cget() {
            let ev: KeyboardEvent = ev.unchecked_into();
            let c = char::from_u32(ev.key_code()).unwrap();
            match c {
                'T' => {
                    // 't'
                    log::info!("active trace is {:?}", ctx.opt_active_trace());
                    log::info!("restriction is {:?}", ctx.presentation_signal.get())
                }
                'C' => {
                    // 't'
                    // log::info!("figure context is \n:{:?}", ctx);
                    // log::info!("fcous context is \n:{:?}", self.restriction_context);
                    log::info!("opt active trace id is \n:{:?}", ctx.opt_active_trace_id());
                }
                'F' => {
                    // log::info!(
                    //     "figure canvases: {:#?}",
                    //     ctx.figure_canvases.borrow(file!(), line!())
                    // );
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

impl DeveloperGuiContext {
    pub(super) fn handle_status_change(&'static self, event: StatusChange) {
        // todo: record user events
        match event {
            StatusChange::ToggleExpansion { trace_id } => self.toggle_expansion(trace_id),
            StatusChange::ToggleShown { trace_id } => self.toggle_shown(trace_id),
            StatusChange::Activate { trace_id } => self.activate(trace_id),
            StatusChange::SetPresentation {
                presentation: restriction,
            } => self.set_restriction(restriction),
            StatusChange::TogglePin { trace_id } => self.toggle_pin(trace_id),
        }
    }

    pub fn toggle_expansion_handler(&'static self, trace_id: TraceId) -> Rc<dyn Fn()> {
        Rc::new(move || self.handle_status_change(StatusChange::ToggleExpansion { trace_id }))
    }

    pub fn toggle_shown_handler(&'static self, trace_id: TraceId) -> impl Fn() {
        move || self.handle_status_change(StatusChange::ToggleShown { trace_id })
    }

    pub fn toggle_pin_handler(&'static self, trace_id: TraceId) -> impl Fn() {
        move || self.handle_status_change(StatusChange::TogglePin { trace_id })
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
            dialog.show_modal();
        }
    }

    pub(crate) fn set_sample_id_handler(&'static self, sample_id: SampleId) -> impl Fn(Event) {
        move |_| {
            self.handle_status_change(StatusChange::update_restriction(self, |res| {
                res.set_sample_id(sample_id)
            }))
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
                            let new_partition = Partition {
                                ncol,
                                variant: PartitionVariant::Label(Label(label_raw)),
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

    pub fn shrink_partition_handler(&'static self, idx: usize) -> impl Fn(Event) {
        move |_| {
            self.handle_status_change(StatusChange::update_restriction(self, |res| {
                res.shrink_partition(idx)
            }))
        }
    }

    pub fn expand_partition_handler(&'static self, idx: usize) -> impl Fn(Event) {
        move |_| {
            self.handle_status_change(StatusChange::update_restriction(self, |res| {
                res.expand_partition(idx)
            }))
        }
    }

    pub fn remove_partition_handler(&'static self, idx: usize) -> impl Fn(Event) {
        move |_| {
            self.handle_status_change(StatusChange::update_restriction(self, |res| {
                res.remove_partition(idx)
            }))
        }
    }
}
