use super::*;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, KeyboardEvent};

impl DebuggerContext {
    // pub(super) fn toggle_restriction_kind(&'static self) {
    //     let mut restriction = self.restriction_context.restriction.cget();
    //     restriction.toggle_is_specific();
    //     self.set_restriction(restriction)
    // }

    pub(super) fn set_restriction(&'static self, new_restriction: Restriction) {
        let opt_active_trace_id = self.trace_context.opt_active_trace_id.cget();
        let needs_figure_canvas_data =
            self.needs_figure_canvas_data(opt_active_trace_id, &new_restriction);
        log::info!("needs_figure_canvas_data: {:?}", needs_figure_canvas_data);
        let needs_figure_control_data =
            self.needs_figure_control_data(opt_active_trace_id, &new_restriction);
        let needs_stalk = self.needs_stalk(opt_active_trace_id, &new_restriction);
        let needs_response = needs_figure_canvas_data || needs_figure_control_data || needs_stalk;
        self.ws.send_message(
            HuskyTracerGuiMessageVariant::SetRestriction {
                restriction: new_restriction.clone(),
                needs_figure_canvas_data,
                needs_figure_control_data,
                needs_stalk,
            },
            if needs_response {
                Some(Box::new(move |message| match message.variant {
                    HuskyTracerServerMessageVariant::SetRestriction {
                        opt_figure_canvas_data,
                        opt_figure_control_data,
                        new_trace_stalks,
                    } => {
                        opt_active_trace_id.map(|active_trace_id| {
                            let active_trace = self.trace_context.trace_data(active_trace_id);
                            self.figure_context.set_opt_figure_data(
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
    //                 get_element_by_id::<HtmlDialogElement>("restriction-dialog");
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
