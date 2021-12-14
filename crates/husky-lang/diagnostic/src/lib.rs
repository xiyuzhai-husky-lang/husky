mod collect;
mod kind;
mod query;
mod reserver;
mod severity;

pub use kind::DiagnosticKind;
pub use query::{DiagnosticQuery, DiagnosticQueryStorage};
pub use severity::Severity;

use std::sync::Arc;

use common::*;

use text_size::{TextRange, TextSize};

use collect::collect_diagnostics;
use reserver::DiagnosticReserve;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub range: TextRange,
    pub message: String,
    pub code: String,
    pub kind: DiagnosticKind,
}

impl Diagnostic {
    pub fn todo() -> Diagnostic {
        Diagnostic {
            severity: Severity::Error,
            range: TextRange::new(TextSize::from(0), TextSize::from(10)),
            message: "todo".into(),
            code: "todo".into(),
            kind: DiagnosticKind::Todo,
        }
    }
}
