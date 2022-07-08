mod attention_context;
mod figure_context;
mod impl_init;
mod impl_user_event;
mod internal;
mod trace_context;
mod utils;

pub(crate) use trace_context::*;

use crate::{services::websocket::WebsocketService, *};
use attention_context::*;
use figure_context::*;
use futures::{channel::mpsc::Sender, stream::SplitStream, SinkExt, StreamExt};
use internal::*;
use reqwasm::websocket::{futures::WebSocket, Message};
use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};
use wasm_bindgen_futures::spawn_local;

pub struct DebuggerContext {
    pub ws: WebsocketService,
    pub scope: Scope<'static>,
    pub window_inner_height: &'static Signal<f64>,
    pub window_inner_width: &'static Signal<f64>,
    pub trace_context: TraceContext,
    pub figure_context: FigureContext,
    pub attention_context: AttentionContext,
    pub dialog_opened: &'static Signal<bool>,
}

impl DebuggerContext {
    pub fn new_ref(scope: Scope<'static>) -> &'static DebuggerContext {
        let (mut ws, mut server_notification_receiver) = WebsocketService::new();
        let context =
            unsafe { as_static_ref(provide_context(scope, DebuggerContext::new(scope, ws))) };
        context.init(server_notification_receiver);
        context
    }
}
