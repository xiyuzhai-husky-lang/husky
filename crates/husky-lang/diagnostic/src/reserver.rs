use std::sync::Mutex;

use crate::*;

pub struct DiagnosticReserve {
    diagnostics: Vec<Diagnostic>,
    drained_mu: Mutex<bool>,
}

impl Debug for DiagnosticReserve {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

    pub fn drained(&self) -> bool {
        *self.drained_mu.lock().unwrap()
    }

    pub fn diagnostics_ref(&self) -> &Vec<Diagnostic> {
        &self.diagnostics
    }
}

impl AsRef<Vec<Diagnostic>> for DiagnosticReserve {
    fn as_ref(&self) -> &Vec<Diagnostic> {
        &self.diagnostics
    }
}
