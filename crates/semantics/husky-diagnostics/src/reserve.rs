use std::sync::Mutex;

use crate::*;

pub struct DiagnosticReserve {
    diagnostics: Vec<Diagnostic>,
    drained_mu: Mutex<bool>,
}

impl std::fmt::Debug for DiagnosticReserve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DiagnosticReserve")
            .field("diagnostics", &self.diagnostics)
            .field("drained", &*self.drained_mu.lock().unwrap())
            .finish()
    }
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
            drained_mu: Mutex::new(false),
        }
    }

    pub fn drain<F>(&self, mut f: F)
    where
        F: FnMut(Vec<Diagnostic>) -> (),
    {
        let drained: &mut bool = &mut self.drained_mu.lock().unwrap();
        if !*drained {
            *drained = true;
            f(self.diagnostics.clone())
        }
    }
}
