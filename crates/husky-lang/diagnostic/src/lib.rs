mod collect;
mod kind;
mod query;
mod reserver;
mod severity;

pub use kind::DiagnosticKind;
pub use query::{DiagnosticQuery, DiagnosticQueryStorage};
pub use severity::DiagnosticSeverity;

use std::sync::Arc;

use text::TextRange;

use common::*;

use collect::collect_diagnostics;
use reserver::DiagnosticReserve;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Diagnostic {
    severity: DiagnosticSeverity,
    range: TextRange,
    message: String,
    code: &'static str,
    kind: DiagnosticKind,
}

impl From<&scope::ScopeDefError> for Diagnostic {
    fn from(error: &scope::ScopeDefError) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            range: error.range.clone(),
            message: "messagetodo".into(),
            code: error.code(),
            kind: DiagnosticKind::ScopeDefError,
        }
    }
}

impl Into<lsp_types::Diagnostic> for Diagnostic {
    fn into(self) -> lsp_types::Diagnostic {
        lsp_types::Diagnostic {
            range: self.range.into(),
            severity: Some(self.severity.into()),
            code: Some(lsp_types::NumberOrString::String(self.code.into())),
            code_description: None,
            source: Some("husky-lang-server".to_string()),
            message: self.message,
            related_information: None,
            tags: None,
            data: None,
        }
    }
}
