mod collect;
mod kind;
mod query;
// mod reserve;
mod severity;

use ast::{AstError, AstErrorVariant};
use dev_utils::DevSource;
use infer_error::{InferError, InferErrorVariant};
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
    dev_src: DevSource,
}

impl From<&EntityDefnError> for Diagnostic {
    fn from(error: &EntityDefnError) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            range: error.range.clone(),
            message: error.message(),
            dev_src: error.dev_src.clone(),
        }
    }
}

impl From<&AstError> for Diagnostic {
    fn from(error: &AstError) -> Self {
        match error.variant {
            AstErrorVariant::Original { ref message, range } => Self {
                severity: DiagnosticSeverity::Error,
                range: range.clone(),
                message: format!("Ast Error: {}", message),
                dev_src: error.dev_src.clone(),
            },
            AstErrorVariant::Derived => todo!(),
        }
    }
}

impl From<&InferError> for Diagnostic {
    fn from(error: &InferError) -> Self {
        match error.variant {
            InferErrorVariant::Derived => panic!(),
            InferErrorVariant::Original { ref message, range } => Self {
                severity: DiagnosticSeverity::Error,
                range: range.clone(),
                message: format!("Infer Error: {}", message),
                dev_src: error.dev_src.clone(),
            },
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
