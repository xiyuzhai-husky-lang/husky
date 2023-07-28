mod figure_context;
mod impl_status_change;
mod init;
mod presentation;
mod process_change;
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
    // visibility
    pub(crate) visibility: Scope,
    // hidden state
    pub(crate) window_inner_height: &'static Signal<f64>,
    pub(crate) window_inner_width: &'static Signal<f64>,
    pub(crate) dialog_opened: &'static Signal<bool>,
    // trace
    trace_nodes: InformativeRefCell<Vec<TraceNodeState>>,
    subtrace_ids_map: InformativeRefCell<HashMap<SubtracesKey, &'static [TraceId]>>,
    trace_stalks: InformativeRefCell<HashMap<TraceStalkKey, &'static TraceStalk>>,
    trace_statss: InformativeRefCell<HashMap<TraceStatsKey, Option<&'static TraceStats>>>,
    root_trace_ids_signal: &'static Signal<Vec<TraceId>>,
    trace_listing: &'static Signal<Vec<TraceId>>,
    // figure
    specific_figure_canvases:
        InformativeRefCell<HashMap<SpecificFigureCanvasKey, &'static SpecificFigureCanvasData>>,
    generic_figure_canvases:
        InformativeRefCell<HashMap<GenericFigureCanvasKey, &'static GenericFigureCanvasData>>,
    // todo
    // figure_canvas_atoms: &'static FigureCanvasAtomArena,
    figure_controls:
        InformativeRefCell<HashMap<FigureControlKey, &'static Signal<FigureControlData>>>,
    presentation_signal: &'static Signal<Presentation>,
}

impl DeveloperGuiContext {
    pub fn new_ref(visibility: Scope) -> &'static DeveloperGuiContext {
        let (mut ws, mut server_notification_receiver) = WebsocketService::new();
        let context = unsafe {
            as_leash(provide_context(
                visibility,
                DeveloperGuiContext::new(visibility, ws),
            ))
        };
        context.init(server_notification_receiver);
        context
    }

    fn new(visibility: Scope, ws: WebsocketService) -> DeveloperGuiContext {
        let window = web_sys::window().unwrap();
        let window_inner_height =
            create_signal(visibility, window.inner_height().unwrap().as_f64().unwrap());
        let window_inner_width =
            create_signal(visibility, window.inner_width().unwrap().as_f64().unwrap());
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
        let presentation_signal = &create_static_signal(visibility, Presentation::default());
        DeveloperGuiContext {
            ws,
            visibility,
            // hidden state
            window_inner_height,
            window_inner_width,
            dialog_opened: create_signal(visibility, false),
            // trace
            trace_nodes: Default::default(),
            subtrace_ids_map: Default::default(),
            trace_stalks: Default::default(),
            trace_statss: Default::default(),
            root_trace_ids_signal: create_signal(visibility, vec![]),
            trace_listing: create_signal(visibility, vec![]),
            // figure
            specific_figure_canvases: Default::default(),
            generic_figure_canvases: Default::default(),
            // figure_canvas_atoms: todo!(),
            figure_controls: Default::default(),
            // user state
            presentation_signal,
        }
    }
}
