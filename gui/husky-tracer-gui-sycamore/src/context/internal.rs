use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;

use super::*;
pub struct TracerContextInternal {
    pub ws: WebsocketService,
    pub window_inner_height: Rc<Signal<f64>>,
    pub window_inner_width: Rc<Signal<f64>>,
    pub tree_context: TreeContext,
    pub figure_context: FigureContext,
    pub focus_context: FocusContext,
}

impl TracerContextInternal {
    pub fn new(ws: WebsocketService) -> TracerContextInternal {
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
                    log::info!("resized");
                    window_inner_height.set(window.inner_height().unwrap().as_f64().unwrap());
                    window_inner_width.set(window.inner_width().unwrap().as_f64().unwrap());
                }) as Box<dyn FnMut(_)>)
            };
            window
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
        TracerContextInternal {
            window_inner_height,
            window_inner_width,
            ws,
            tree_context: Default::default(),
            figure_context: Default::default(),
            focus_context: Default::default(),
        }
    }

    pub fn toggle_expansion(&self, trace_id: TraceId) {
        let expansion = self.tree_context.expanded_signal(trace_id);
        if expansion.get_cloned() {
            expansion.set(false)
        } else {
            let focus = self.focus_context.focus_signal.get();
            let trace_kind = self.tree_context.trace_kind(trace_id);
            let key = SubtracesKey::new(&focus, trace_kind, trace_id);
            if self.tree_context.subtraces_map.contains_key(&key) {
                self.ws.send_message(
                    HuskyTracerGuiMessageVariant::ToggleExpansion {
                        trace_id,
                        request_subtraces: false,
                    },
                    None,
                );
                expansion.set(true)
            } else {
                self.ws.send_message(
                    HuskyTracerGuiMessageVariant::ToggleExpansion {
                        trace_id,
                        request_subtraces: true,
                    },
                    Some(Box::new(|message| match message.variant {
                        HuskyTracerServerMessageVariant::ToggleExpansion {
                            subtraces,
                            associated_traces,
                        } => todo!(),
                        _ => panic!(),
                    })),
                )
            }
        }
    }
}
