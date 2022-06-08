mod internal;

use crate::*;
use internal::*;
use std::{cell::RefCell, rc::Rc, sync::Mutex};

#[derive(Debug)]
pub struct DebuggerContext {
    internal: Rc<Mutex<DebuggerContextInternal>>,
}

impl PartialEq for DebuggerContext {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl DebuggerContext {
    pub fn new() -> Rc<DebuggerContext> {
        Rc::new(DebuggerContext {
            internal: Rc::new(Mutex::new(DebuggerContextInternal::new())),
        })
    }

    pub fn get_store(&self) -> Store<i32> {
        self.internal.lock().unwrap().get_store()
    }
}
