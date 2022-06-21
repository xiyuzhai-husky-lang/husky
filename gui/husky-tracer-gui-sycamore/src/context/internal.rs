use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;

use super::*;

#[derive(Debug)]
pub struct DebuggerContextInternal {
    pub ws: WebsocketService,
    pub window_inner_height: Rc<Signal<f64>>,
    pub window_inner_width: Rc<Signal<f64>>,
    pub trace_context: TraceContext,
    pub figure_context: FigureContext,
    pub attention_context: AttentionContext,
    pub dialog_opened: Rc<Signal<bool>>,
}

impl DebuggerContextInternal {
    pub fn new(ws: WebsocketService) -> DebuggerContextInternal {
        let window = web_sys::window().unwrap();
        let window_inner_height = Rc::new(Signal::new(
            window.inner_height().unwrap().as_f64().unwrap(),
        ));
        let window_inner_width =
            Rc::new(Signal::new(window.inner_width().unwrap().as_f64().unwrap()));
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
        DebuggerContextInternal {
            window_inner_height,
            window_inner_width,
            ws,
            trace_context: Default::default(),
            figure_context: Default::default(),
            attention_context: Default::default(),
            dialog_opened: Rc::new(Signal::new(false)),
        }
    }
}
