mod figure_context;
mod focus_context;
mod impl_handle_server_message;
mod impl_init;
mod impl_listening;
mod trace_context;

use crate::{services::websocket::WebsocketService, *};
use figure_context::*;
use focus_context::*;
use futures::{channel::mpsc::Sender, stream::SplitStream, SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};
use trace_context::*;
use wasm_bindgen_futures::spawn_local;

pub struct DebuggerContext {
    ws: WebsocketService,
    call_backs: HashMap<usize, Box<dyn FnOnce(&mut Self, DebuggerServerMessage)>>,
    trace_context: TraceContext,
    figure_context: FigureContext,
    focus_context: FocusContext,
    pub signal: Signal<i32>,
}

impl DebuggerContext {
    pub fn new_raw(ws: WebsocketService) -> DebuggerContext {
        DebuggerContext {
            signal: Signal::new(0),
            ws,
            call_backs: Default::default(),
            trace_context: Default::default(),
            figure_context: Default::default(),
            focus_context: Default::default(),
        }
    }

    pub fn new() -> Rc<RefCell<DebuggerContext>> {
        let (mut ws, mut read) = WebsocketService::new();
        let context = Rc::new(RefCell::new(DebuggerContext::new_raw(ws.clone())));
        DebuggerContext::spawn_listening(context.clone(), read);
        context.borrow_mut().request_init();
        context
    }
}
