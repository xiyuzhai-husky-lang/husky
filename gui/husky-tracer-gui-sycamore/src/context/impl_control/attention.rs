use super::*;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, KeyboardEvent};

impl DebuggerContext {
    pub(super) fn toggle_attention_kind(&'static self) {
        self.set_attention(self.attention_context.toggled_attention_kind())
    }

    fn set_attention(&'static self, attention: Attention) {
        let opt_active_trace_id = self.trace_context.opt_active_trace_id.cget();
        let request_figure = match opt_active_trace_id {
            Some(active_trace_id) => {
                let active_trace = self.trace_context.trace(active_trace_id);
                !self
                    .figure_context
                    .is_figure_cached(&active_trace, &attention)
            }
            None => false,
        };
        let request_stalk = attention.opt_sample_id().is_some();
        if request_figure || request_stalk {
            self.ws.send_message(
                HuskyTracerGuiMessageVariant::LockAttention {
                    attention: attention.clone(),
                    request_figure,
                    request_stalk,
                },
                Some(Box::new(move |message| match message.variant {
                    HuskyTracerServerMessageVariant::LockAttention {
                        opt_figure_data,
                        new_trace_stalks,
                    } => {
                        opt_figure_data.map(|(figure_canvas_data, figure_control_data)| {
                            let active_trace =
                                self.trace_context.trace(opt_active_trace_id.unwrap());
                            self.figure_context.set_figure(
                                &active_trace,
                                &attention,
                                figure_canvas_data,
                                figure_control_data,
                            )
                        });
                        self.trace_context.receive_trace_stalks(
                            new_trace_stalks
                                .into_iter()
                                .map(|(k, v)| (k, self.create_static_ref(v))),
                        );
                        self.attention_context.attention.set(attention.clone());
                    }
                    _ => panic!(),
                })),
            )
        } else {
            self.set_attention_without_request(attention)
        }
    }

    fn set_attention_without_request(&self, attention: Attention) {
        self.attention_context.attention.set(attention.clone());
        self.ws.send_message(
            HuskyTracerGuiMessageVariant::LockAttention {
                attention,
                request_figure: false,
                request_stalk: false,
            },
            None,
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
