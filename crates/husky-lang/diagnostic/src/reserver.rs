use std::sync::{Arc, Mutex};

use crate::*;

#[derive(Debug)]
pub struct DiagnosticReserve {
    diagnostics: Vec<Diagnostic>,
    drained: Mutex<bool>,
}

impl PartialEq for DiagnosticReserve {
    fn eq(&self, other: &Self) -> bool {
        self.diagnostics == other.diagnostics
    }
}
impl Eq for DiagnosticReserve {}

impl DiagnosticReserve {
    pub fn new(diagnostics: Vec<Diagnostic>) -> Self {
        Self {
            diagnostics,
            drained: Mutex::new(false),
        }
    }

    pub fn drain<F>(&self, mut f: F)
    where
        F: FnMut(Vec<Diagnostic>) -> (),
    {
        let drained: &mut bool = &mut self.drained.lock().unwrap();
        if !*drained {
            *drained = true;
            f(self.diagnostics.clone())
        }
    }
}
