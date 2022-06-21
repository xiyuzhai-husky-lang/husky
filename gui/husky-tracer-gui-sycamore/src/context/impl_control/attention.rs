use super::*;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, KeyboardEvent};

impl DebuggerContext {
    pub(super) fn toggle_attention_kind(&self) {
        self.set_attention(self.attention_context.toggled_focus_kind())
    }

    fn set_attention(&self, attention: Focus) {
        match self.trace_context.opt_active_trace_id.cget() {
            Some(active_trace_id) => {
                let active_trace = self.trace_context.trace(active_trace_id);
                let request_figure = !self
                    .figure_context
                    .is_figure_cached(&active_trace, &attention);
                if request_figure {
                    let this = self.clone();
                    self.ws.send_message(
                        HuskyTracerGuiMessageVariant::LockFocus {
                            focus: attention.clone(),
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
                                    &attention,
                                    figure_canvas_data,
                                    figure_control_data,
                                );
                                this.attention_context.focus.set(attention.clone());
                            }
                            _ => panic!(),
                        })),
                    )
                } else {
                    self.set_attention_without_request(attention)
                }
            }
            None => self.set_attention_without_request(attention),
        };
    }

    fn set_attention_without_request(&self, focus: Focus) {
        self.attention_context.focus.set(focus.clone());
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

    pub(super) fn set_attention_from_dialog(&self) {
        let sample_id_value = get_element_by_id::<HtmlInputElement>("sample-id-input").value();
        match sample_id_value.parse::<usize>() {
            Ok(sample_id) => {
                self.set_attention(Focus::Specific {
                    input_id: sample_id,
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
