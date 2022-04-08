mod collect;
mod kind;
mod query;
// mod reserve;
mod severity;

use ast::AstError;
pub use kind::DiagnosticKind;
pub use query::{DiagnosticQuery, DiagnosticQueryStorage};
pub use severity::DiagnosticSeverity;

use entity_route_query::EntityDefnError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use text::TextRange;

use collect::collect_diagnostics;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    severity: DiagnosticSeverity,
    range: TextRange,
    message: String,
    kind: DiagnosticKind,
}

impl From<&EntityDefnError> for Diagnostic {
    fn from(error: &EntityDefnError) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            range: error.range.clone(),
            message: error.message(),
            kind: DiagnosticKind::ScopeDefError,
        }
    }
}

impl From<&AstError> for Diagnostic {
    fn from(error: &AstError) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            range: error.range.clone(),
            message: error.message(),
            kind: DiagnosticKind::ScopeDefError,
        }
    }
}

impl Into<lsp_types::Diagnostic> for Diagnostic {
    fn into(self) -> lsp_types::Diagnostic {
        lsp_types::Diagnostic {
            range: self.range.into(),
            severity: Some(self.severity.into()),
            code: None,
            code_description: None,
            source: Some("husky-analyzer".to_string()),
            message: self.message,
            related_information: None,
            tags: None,
            data: None,
        }
    }
}
