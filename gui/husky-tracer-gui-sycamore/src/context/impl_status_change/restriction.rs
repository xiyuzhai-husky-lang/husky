use super::*;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, KeyboardEvent};

impl DebuggerContext {
    // pub(super) fn toggle_restriction_kind(&'static self) {
    //     let mut restriction = self.restriction_context.restriction.cget();
    //     restriction.toggle_is_specific();
    //     self.set_restriction(restriction)
    // }

    #[cfg(feature = "verify_consistency")]
    pub(super) fn set_restriction(&'static self, new_restriction: Presentation) {
        let opt_active_trace_id = self.trace_context.opt_active_trace_id.cget();
        let needs_figure_canvases =
            self.needs_figure_canvases(opt_active_trace_id, &new_restriction);
        let needs_figure_controls =
            self.needs_figure_controls(opt_active_trace_id, &new_restriction);
        let new_stalk_keys = self.new_stalk_keys(&new_restriction);
        let new_stats_keys = self.new_stats_keys(&new_restriction);
        // todo: make this into an optional feature named "check"
        let needs_stalk = new_stalk_keys.len() > 0;
        let needs_stats = new_stats_keys.len() > 0;
        let needs_response =
            needs_figure_canvases || needs_figure_controls || needs_stalk || needs_stats;
        self.ws.send_message(
            HuskyTracerGuiMessageVariant::SetRestriction {
                restriction: new_restriction.clone(),
                needs_figure_canvases,
                needs_figure_controls,
                new_stalk_keys,
                new_stats_keys,
            },
            if needs_response {
                Some(Box::new(move |message| match message.variant {
                    HuskyTracerServerMessageVariant::SetRestriction {
                        opt_figure_canvas_data,
                        opt_figure_control_data,
                        new_trace_stalks,
                        new_trace_stats,
                    } => {
                        opt_active_trace_id.map(|active_trace_id| {
                            let active_trace = self.trace_context.trace_data(active_trace_id);
                            self.set_opt_figure_data(
                                self.scope,
                                &active_trace,
                                &new_restriction,
                                opt_figure_canvas_data
                                    .map(|figure_canvas_data| self.alloc_value(figure_canvas_data)),
                                opt_figure_control_data,
                            )
                        });
                        self.trace_context.receive_trace_stalks(
                            new_trace_stalks
                                .into_iter()
                                .map(|(k, v)| (k, self.alloc_value(v))),
                        );
                        self.trace_context.receive_trace_stats(
                            new_trace_stats
                                .into_iter()
                                .map(|(k, v)| (k, v.map(|v| self.alloc_value(v)))),
                        );
                        self.restriction_context
                            .restriction
                            .set(new_restriction.clone())
                    }
                    _ => panic!(),
                }))
            } else {
                self.restriction_context
                    .restriction
                    .set(new_restriction.clone());
                None
            },
        );
    }

    #[cfg(not(feature = "verify_consistency"))]
    pub(super) fn set_restriction(&'static self, new_restriction: Presentation) {
        let opt_active_trace_id = self.trace_context.opt_active_trace_id.cget();
        let needs_figure_canvases =
            self.needs_figure_canvases(opt_active_trace_id, &new_restriction);
        let needs_figure_controls =
            self.needs_figure_controls(opt_active_trace_id, &new_restriction);
        let needs_stalks = self.needs_stalks(&new_restriction);
        let needs_statss = self.needs_statss(&new_restriction);
        let needs_response =
            needs_figure_canvases || needs_figure_controls || needs_stalks || needs_statss;
        self.ws.send_message(
            HuskyTracerGuiMessageVariant::SetRestriction {
                restriction: new_restriction.clone(),
                needs_figure_canvases,
                needs_figure_controls,
                needs_stalks,
                needs_statss,
            },
            if needs_response {
                Some(Box::new(move |message| match message.variant {
                    HuskyTracerServerMessageVariant::SetRestriction {
                        new_figure_canvases,
                        new_figure_controls,
                        new_trace_stalks,
                        new_trace_statss,
                    } => {
                        self.receive_figure_canvases(
                            self.scope,
                            new_figure_canvases
                                .into_iter()
                                .map(|(k, v)| (k, self.alloc_value(v))),
                        );
                        self.receive_figure_controls(self.scope, new_figure_controls.into_iter());
                        self.trace_context.receive_trace_stalks(
                            new_trace_stalks
                                .into_iter()
                                .map(|(k, v)| (k, self.alloc_value(v))),
                        );
                        self.trace_context.receive_trace_stats(
                            new_trace_statss
                                .into_iter()
                                .map(|(k, v)| (k, v.map(|v| self.alloc_value(v)))),
                        );
                        self.restriction_context
                            .restriction
                            .set(new_restriction.clone())
                    }
                    _ => panic!(),
                }))
            } else {
                self.restriction_context
                    .restriction
                    .set(new_restriction.clone());
                None
            },
        );
    }

    // pub(super) fn set_restriction_from_dialog(&'static self) {
    //     let sample_id_value = get_element_by_id::<HtmlInputElement>("sample-id-input").value();
    //     match sample_id_value.parse::<usize>() {
    //         Ok(raw) => {
    //             let mut restriction = self.restriction_context.restriction.cget();
    //             restriction.specific_sample_id = SampleId(raw);
    //             self.set_restriction(restriction);
    //             let restriction_dialog =
    //                 restriction_dialog();
    //             restriction_dialog.close()
    //         }
    //         Err(_) => alert!("`{}` is not a valid sample id", sample_id_value),
    //     }
    // }
}

#[wasm_bindgen]
pub fn sleep(ms: i32) -> js_sys::Promise {
    log::info!("not working");
    js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    })
}
