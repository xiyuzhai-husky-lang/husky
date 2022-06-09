mod internal;

use crate::{*, services::websocket::WebsocketService};
use internal::*;
use wasm_bindgen_futures::spawn_local;
use std::{cell::RefCell, rc::Rc, sync::{Mutex, Arc}};

#[derive(Debug)]
pub struct DebuggerContext {
    pub signal: Signal<i32>,
}

impl DebuggerContext {
    pub fn new_raw() -> DebuggerContext {
        DebuggerContext { signal: Signal::new(0) }
    }

    pub fn new() -> Rc<RefCell<DebuggerContext>>  {
        let context = Rc::new(RefCell::new(DebuggerContext::new_raw()));
        let ws = WebsocketService::new(context.clone());
        {
            let internal = context.clone();
            let a = Rc::new(RefCell::new(0));
            let b = a.clone();
            spawn_local(async move { a; });
        }
        context
    }

    pub(crate) fn process_response(&mut self, response: &str) {
        log::warn!("todo: process_response")
    }
}
