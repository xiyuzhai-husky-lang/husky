use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use super::*;
pub struct TracerContextInternal {
    pub(super) ws: WebsocketService,
    pub(super) call_backs: RefCell<HashMap<usize, Box<dyn FnOnce(&Self, DebuggerServerMessage)>>>,
    pub tree_context: TreeContext,
    pub figure_context: FigureContext,
    pub focus_context: FocusContext,
    pub signal: Signal<i32>,
}

impl TracerContextInternal {
    pub fn new(ws: WebsocketService) -> TracerContextInternal {
        let window = web_sys::window().unwrap();
        {
            let closure = {
                let window = window.clone();
                Closure::wrap(Box::new(move |_event: web_sys::UiEvent| {
                    log::info!(
                        "window size changed to {} {}",
                        window.inner_height().unwrap().as_f64().unwrap(),
                        window.inner_width().unwrap().as_f64().unwrap()
                    );
                }) as Box<dyn FnMut(_)>)
            };
            window
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
        TracerContextInternal {
            signal: Signal::new(0),
            ws,
            call_backs: Default::default(),
            tree_context: Default::default(),
            figure_context: Default::default(),
            focus_context: Default::default(),
        }
    }
}
