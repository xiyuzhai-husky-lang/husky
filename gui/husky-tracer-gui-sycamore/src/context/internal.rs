use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;

use super::*;

impl DebuggerContext {
    pub fn new(scope: Scope<'static>, ws: WebsocketService) -> DebuggerContext {
        let window = web_sys::window().unwrap();
        let window_inner_height =
            create_signal(scope, window.inner_height().unwrap().as_f64().unwrap());
        let window_inner_width =
            create_signal(scope, window.inner_width().unwrap().as_f64().unwrap());
        {
            let window = window.clone();
            let window_inner_height = window_inner_height.clone();
            let window_inner_width = window_inner_width.clone();
            let closure = {
                let window = window.clone();
                Closure::wrap(Box::new(move |_event: web_sys::UiEvent| {
                    window_inner_height.set(window.inner_height().unwrap().as_f64().unwrap());
                    window_inner_width.set(window.inner_width().unwrap().as_f64().unwrap());
                }) as Box<dyn FnMut(_)>)
            };
            window
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
        DebuggerContext {
            window_inner_height,
            window_inner_width,
            ws,
            trace_context: TraceContext::new(scope),
            figure_context: FigureContext::new(scope),
            restriction_context: RestrictionContext::new(scope),
            dialog_opened: create_signal(scope, false),
            scope,
        }
    }
}
