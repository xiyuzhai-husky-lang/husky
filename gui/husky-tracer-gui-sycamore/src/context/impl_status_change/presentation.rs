use super::*;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, KeyboardEvent};

impl DeveloperGuiContext {
    pub(super) fn try_set_presentation(&'static self, new_presentation: Presentation) {
        let opt_active_trace_id = self.opt_active_trace_id();
        let needs_figure_canvases =
            self.needs_figure_canvases(opt_active_trace_id, &new_presentation);
        let needs_figure_controls =
            self.needs_figure_controls(opt_active_trace_id, &new_presentation);
        let needs_stalks = self.needs_stalks(&new_presentation);
        let needs_statss = self.needs_statss(&new_presentation);
        let needs_response =
            needs_figure_canvases || needs_figure_controls || needs_stalks || needs_statss;
        self.ws.try_apply_change(
            HuskyTracerGuiMessageVariant::SetPresentation {
                presentation: new_presentation.clone(),
                needs_figure_canvases,
                needs_figure_controls,
                needs_stalks,
                needs_statss,
            },
            needs_response,
            || self.set_presentation(new_presentation),
        )
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
