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
    // ws
    pub(crate) ws: WebsocketService,
    // scope
    pub(crate) scope: Scope<'static>,
    // hidden state
    pub(crate) window_inner_height: &'static Signal<f64>,
    pub(crate) window_inner_width: &'static Signal<f64>,
    pub(crate) dialog_opened: &'static Signal<bool>,
    // trace
    trace_nodes: RefCell<Vec<TraceNodeState>>,
    subtrace_ids_map: RefCell<HashMap<SubtracesKey, &'static [TraceId]>>,
    trace_stalks: RefCell<HashMap<TraceStalkKey, &'static TraceStalk>>,
    trace_statss: RefCell<HashMap<TraceStatsKey, Option<&'static TraceStats>>>,
    root_trace_ids_signal: &'static Signal<Vec<TraceId>>,
    trace_listing: &'static Signal<Vec<TraceId>>,
    // figure
    specific_figure_canvases:
        RefCell<HashMap<SpecificFigureCanvasKey, &'static SpecificFigureCanvasData>>,
    generic_figure_canvases:
        RefCell<HashMap<GenericFigureCanvasKey, &'static GenericFigureCanvasData>>,
    figure_canvas_atoms: &'static FigureCanvasAtomArena,
    figure_controls: RefCell<HashMap<FigureControlKey, &'static Signal<FigureControlData>>>,
    presentation_signal: &'static Signal<Presentation>,
    // global control
    pub(crate) presentation_locked_signal: Signal<bool>,
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
        DeveloperGuiContext {
            ws,
            scope,
            // hidden state
            window_inner_height,
            window_inner_width,
            dialog_opened: create_signal(scope, false),
            // trace
            trace_nodes: Default::default(),
            subtrace_ids_map: Default::default(),
            trace_stalks: Default::default(),
            trace_statss: Default::default(),
            root_trace_ids_signal: create_signal(scope, vec![]),
            trace_listing: create_signal(scope, vec![]),
            // figure
            specific_figure_canvases: Default::default(),
            generic_figure_canvases: Default::default(),
            figure_canvas_atoms: todo!(),
            figure_controls: Default::default(),
            // user state
            presentation_signal,
            presentation_locked_signal: Default::default(),
        }
    }
}
