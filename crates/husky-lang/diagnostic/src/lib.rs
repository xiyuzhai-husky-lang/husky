use std::sync::{Arc, Mutex};

use common::*;

use text_size::{TextRange, TextSize};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Severity {
    Error,
    // We don't actually emit this one yet, but we should at some point.
    // Warning,
    WeakWarning,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DiagnosticVariant {
    Todo,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub range: TextRange,
    pub message: String,
    pub code: String,
    pub variant: DiagnosticVariant,
}

impl Diagnostic {
    pub fn todo() -> Diagnostic {
        Diagnostic {
            severity: Severity::Error,
            range: TextRange::new(TextSize::from(0), TextSize::from(10)),
            message: "todo".into(),
            code: "todo".into(),
            variant: DiagnosticVariant::Todo,
        }
    }
}

pub struct FileDiagnostics {
    pub file_id: file::FileId,
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::query_group(DiagnosticQueryStorage)]
pub trait DiagnosticQuery {
    fn diagnostic_reserve(&self, id: file::FileId) -> Arc<DiagnosticReserve>;
}

fn diagnostic_reserve(this: &dyn DiagnosticQuery, id: file::FileId) -> Arc<DiagnosticReserve> {
    todo!()
}

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

    pub fn drain(&self) -> Option<Vec<Diagnostic>> {
        let drained: &mut bool = &mut self.drained.lock().unwrap();
        if *drained {
            None
        } else {
            *drained = true;
            Some(self.diagnostics.clone())
        }
    }
}
