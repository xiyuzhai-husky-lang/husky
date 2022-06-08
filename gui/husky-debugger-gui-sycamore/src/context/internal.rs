use crate::services::websocket::WebsocketService;
use crate::*;

#[derive(Debug)]
pub struct DebuggerContextInternal {
    ws: WebsocketService,
    store: Store<i32>,
}

impl DebuggerContextInternal {
    pub fn new() -> DebuggerContextInternal {
        DebuggerContextInternal {
            ws: WebsocketService::new(),
            store: Store::new(0),
        }
    }
    pub fn get_store(&self) -> Store<i32> {
        self.store.clone()
    }
}
