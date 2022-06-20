mod figure_context;
mod focus_context;
mod impl_control;
mod impl_init;
mod internal;
mod trace_context;

pub(crate) use trace_context::*;

use crate::{services::websocket::WebsocketService, *};
use figure_context::*;
use focus_context::*;
use futures::{channel::mpsc::Sender, stream::SplitStream, SinkExt, StreamExt};
use internal::*;
use reqwasm::websocket::{futures::WebSocket, Message};
use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Clone)]
pub struct DebuggerContext(Rc<DebuggerContextInternal>);

impl DebuggerContext {
    pub fn new() -> DebuggerContext {
        let (mut ws, mut server_notification_receiver) = WebsocketService::new();
        let context = DebuggerContext(Rc::new(DebuggerContextInternal::new(ws.clone())));
        context.init(server_notification_receiver);
        context
    }
}

impl std::ops::Deref for DebuggerContext {
    type Target = DebuggerContextInternal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
