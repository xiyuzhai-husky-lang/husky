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
    pub source_id: file::FileId,
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::query_group(DiagnosticQueryGroupStorage)]
pub trait DiagnosticQueryGroup {
    fn diagnostics(&self) -> DiagnosticReserve;
}

fn diagnostics(this: &dyn DiagnosticQueryGroup) -> DiagnosticReserve {
    todo!()
}

type DiagnosticCollection = Vec<(file::FileId, Vec<Diagnostic>)>;

#[derive(Debug, Clone)]
pub struct DiagnosticReserve {
    diagnostics: Arc<Mutex<DiagnosticCollection>>,
}

impl PartialEq for DiagnosticReserve {
    fn eq(&self, other: &Self) -> bool {
        (&self.diagnostics.lock().unwrap() as &DiagnosticCollection)
            == (&other.diagnostics.lock().unwrap() as &DiagnosticCollection)
    }
}
impl Eq for DiagnosticReserve {}

impl DiagnosticReserve {
    pub fn drain(&self) -> DiagnosticCollection {
        std::mem::take(&mut self.diagnostics.lock().unwrap())
    }
}
