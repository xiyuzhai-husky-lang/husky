use super::*;
use web_sys::{Event, KeyboardEvent};

impl TracerContext {
    pub(super) fn toggle_focus_kind(&self) {
        self.set_focus(self.focus_context.toggled_focus_kind())
    }

    fn set_focus(&self, focus: Focus) {
        match self.tree_context.opt_active_trace_id.get_cloned() {
            Some(active_trace_id) => {
                let active_trace = self.tree_context.trace(active_trace_id);
                let request_figure = !self.figure_context.is_figure_cached(&active_trace, &focus);
                if request_figure {
                    let this = self.clone();
                    self.ws.send_message(
                        HuskyTracerGuiMessageVariant::LockFocus {
                            focus: focus.clone(),
                            opt_active_trace_id_for_figure: self
                                .tree_context
                                .opt_active_trace_id
                                .get_cloned(),
                        },
                        Some(Box::new(move |message| match message.variant {
                            HuskyTracerServerMessageVariant::LockFocus {
                                figure,
                                figure_control,
                            } => this.figure_context.set_figure(
                                &active_trace,
                                &focus,
                                figure,
                                figure_control,
                            ),
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
                opt_active_trace_id_for_figure: None,
            },
            None,
        );
    }
}
