use super::*;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, KeyboardEvent};

impl DebuggerContext {
    pub(super) fn toggle_attention_kind(&'static self) {
        self.set_attention(self.attention_context.toggled_attention_kind())
    }

    fn set_attention(&'static self, new_attention: Attention) {
        let opt_active_trace_id = self.trace_context.opt_active_trace_id.cget();
        let need_figure_canvas_data =
            self.need_figure_canvas_data(opt_active_trace_id, &new_attention);
        let need_figure_control_data =
            self.need_figure_control_data(opt_active_trace_id, &new_attention);
        let need_stalk = self.need_stalk(opt_active_trace_id, &new_attention);
        let need_response = need_figure_control_data || need_figure_control_data || need_stalk;
        self.ws.send_message(
            HuskyTracerGuiMessageVariant::LockAttention {
                attention: new_attention.clone(),
                need_figure_canvas_data,
                need_figure_control_data,
                need_stalk,
            },
            if need_response {
                Some(Box::new(move |message| match message.variant {
                    HuskyTracerServerMessageVariant::LockAttention {
                        opt_figure_canvas_data,
                        opt_figure_control_data,
                        new_trace_stalks,
                    } => {
                        let active_trace = self.trace_context.trace(opt_active_trace_id.unwrap());
                        self.figure_context.set_opt_figure_data(
                            self.scope,
                            &active_trace,
                            &new_attention,
                            opt_figure_canvas_data
                                .map(|figure_canvas_data| self.alloc_value(figure_canvas_data)),
                            opt_figure_control_data,
                        );
                        self.trace_context.receive_trace_stalks(
                            new_trace_stalks
                                .into_iter()
                                .map(|(k, v)| (k, self.alloc_value(v))),
                        );
                        self.attention_context.attention.set(new_attention.clone());
                    }
                    _ => panic!(),
                }))
            } else {
                self.attention_context.attention.set(new_attention.clone());
                None
            },
        );
    }

    pub(super) fn set_attention_from_dialog(&'static self) {
        let sample_id_value = get_element_by_id::<HtmlInputElement>("sample-id-input").value();
        match sample_id_value.parse::<usize>() {
            Ok(raw) => {
                self.set_attention(Attention::Specific {
                    sample_id: SampleId(raw),
                });
                let attention_dialog = get_element_by_id::<HtmlDialogElement>("attention-dialog");
                attention_dialog.close()
            }
            Err(_) => alert!("`{}` is not a valid sample id", sample_id_value),
        }
    }
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
