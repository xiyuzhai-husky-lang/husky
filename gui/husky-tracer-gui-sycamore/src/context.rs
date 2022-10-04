mod figure_context;
mod impl_init;
mod impl_status_change;
mod presentation;
mod trace_context;
mod utils;

pub(crate) use trace_context::*;
use vec_like::VecSet;

use crate::{services::websocket::WebsocketService, *};
use figure_context::*;
use futures::{channel::mpsc::Sender, stream::SplitStream, SinkExt, StreamExt};
use presentation::*;
use reqwasm::websocket::{futures::WebSocket, Message};
use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};
use wasm_bindgen_futures::spawn_local;

pub struct DeveloperGuiContext {
    pub(crate) ws: WebsocketService,
    pub(crate) scope: Scope<'static>,
    pub(crate) window_inner_height: &'static Signal<f64>,
    pub(crate) window_inner_width: &'static Signal<f64>,
    pub(crate) trace_context: TraceContext,
    pub(crate) dialog_opened: &'static Signal<bool>,
    figure_canvases: RefCell<HashMap<FigureCanvasKey, &'static FigureCanvasData>>,
    figure_controls: RefCell<HashMap<FigureControlKey, &'static Signal<FigureControlData>>>,
    presentation_signal: &'static Signal<Presentation>,
    opt_sample_id_signal: &'static ReadSignal<Option<SampleId>>,
    opt_active_trace_id_signal: &'static ReadSignal<Option<TraceId>>,
    pub(crate) presentation_locked_signal: Signal<bool>,
    pub(crate) pins_signal: &'static Signal<VecSet<TraceId>>,
}

impl DeveloperGuiContext {
    pub fn new_ref(scope: Scope<'static>) -> &'static DeveloperGuiContext {
        let (mut ws, mut server_notification_receiver) = WebsocketService::new();
        let context =
            unsafe { as_static_ref(provide_context(scope, DeveloperGuiContext::new(scope, ws))) };
        context.init(server_notification_receiver);
        context
    }

    fn new(scope: Scope<'static>, ws: WebsocketService) -> DeveloperGuiContext {
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
        let presentation_signal = &create_static_signal(scope, Presentation::default());
        let opt_sample_id_signal =
            create_static_memo(scope, || presentation_signal.get().opt_sample_id());
        let opt_active_trace_id_signal =
            create_static_memo(scope, || presentation_signal.get().opt_active_trace_id());
        DeveloperGuiContext {
            window_inner_height,
            window_inner_width,
            ws,
            trace_context: TraceContext::new(scope),
            dialog_opened: create_signal(scope, false),
            scope,
            figure_canvases: Default::default(),
            figure_controls: Default::default(),
            pins_signal: create_static_signal(scope, Default::default()),
            presentation_signal,
            opt_sample_id_signal,
            opt_active_trace_id_signal,
            presentation_locked_signal: Default::default(),
        }
    }
}
