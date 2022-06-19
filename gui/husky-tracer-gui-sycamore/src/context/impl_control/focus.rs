use super::*;
use web_sys::{Event, KeyboardEvent};

impl TracerContext {
    pub(super) fn toggle_focus_kind(&self) {
        self.set_focus(self.focus_context.toggled_focus_kind())
    }

    fn set_focus(&self, focus: Focus) {
        match self.tree_context.opt_active_trace_id.cget() {
            Some(active_trace_id) => {
                let active_trace = self.tree_context.trace(active_trace_id);
                let request_figure = !self.figure_context.is_figure_cached(&active_trace, &focus);
                log::info!("handle stalk");
                if request_figure {
                    let this = self.clone();
                    self.ws.send_message(
                        HuskyTracerGuiMessageVariant::LockFocus {
                            focus: focus.clone(),
                            opt_active_trace_id_for_request: Some(active_trace_id),
                            request_figure,
                            request_stalk: false,
                        },
                        Some(Box::new(move |message| match message.variant {
                            HuskyTracerServerMessageVariant::LockFocus {
                                figure_canvas_data,
                                figure_control_data,
                            } => {
                                this.figure_context.set_figure(
                                    &active_trace,
                                    &focus,
                                    figure_canvas_data,
                                    figure_control_data,
                                );
                                log::info!("here1");
                                this.focus_context.focus.set(focus);
                                log::info!("here2");
                            }
                            _ => panic!(),
                        })),
                    )
                } else {
                    self.set_focus_without_request(focus)
                }
            }
            None => self.set_focus_without_request(focus),
        };
    }

    fn set_focus_without_request(&self, focus: Focus) {
        self.focus_context.focus.set(focus.clone());
        self.ws.send_message(
            HuskyTracerGuiMessageVariant::LockFocus {
                focus,
                opt_active_trace_id_for_request: None,
                request_figure: false,
                request_stalk: false,
            },
            None,
        );
    }
}
