mod figure_context;
mod focus_context;
mod impl_init;
mod internal;
mod tree_context;

pub(crate) use tree_context::*;

use crate::{services::websocket::WebsocketService, *};
use figure_context::*;
use focus_context::*;
use futures::{channel::mpsc::Sender, stream::SplitStream, SinkExt, StreamExt};
use internal::*;
use reqwasm::websocket::{futures::WebSocket, Message};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};
use wasm_bindgen_futures::spawn_local;

#[derive(Clone)]
pub struct TracerContext(Rc<TracerContextInternal>);

impl TracerContext {
    pub fn new() -> TracerContext {
        let (mut ws, mut server_notification_receiver) = WebsocketService::new();
        let context = TracerContext(Rc::new(TracerContextInternal::new(ws.clone())));
        context.init(server_notification_receiver);
        context
    }

    pub fn toggle_expansion_handler(&self, trace_id: TraceId) -> Rc<dyn Fn()> {
        let this = self.clone();
        Rc::new(move || this.toggle_expansion(trace_id))
    }
}

impl std::ops::Deref for TracerContext {
    type Target = TracerContextInternal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
