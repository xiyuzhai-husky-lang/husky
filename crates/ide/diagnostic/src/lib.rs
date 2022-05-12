mod collect;
mod kind;
mod query;
// mod reserve;
mod severity;

pub use kind::DiagnosticKind;
pub use query::{DiagnosticQuery, DiagnosticQueryGroupStorage};
pub use severity::DiagnosticSeverity;

use ast::{AstError, AstErrorVariant};
use dev_utils::DevSource;
use entity_route_query::{EntitySyntaxError, EntitySyntaxErrorKind};
use infer_error::{InferError, InferErrorVariant};
use print_utils::p;
use semantics_error::{SemanticError, SemanticErrorVariant};
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::sync::Arc;
use test_utils::{TestDisplay, TestDisplayConfig};
use text::TextRange;
use token::LexError;

use collect::collect_diagnostics;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    severity: DiagnosticSeverity,
    range: TextRange,
    message: String,
    dev_src: DevSource,
}

impl TestDisplay for Diagnostic {
    fn write_inherent(&self, config: TestDisplayConfig, result: &mut String) {
        write!(result, "{:?}\t{}", self.range, self.message).unwrap()
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
            AstErrorVariant::Derived => panic!(),
        }
    }
}

impl From<&InferError> for Diagnostic {
    fn from(error: &InferError) -> Self {
        match error.variant {
            InferErrorVariant::Derived { .. } => {
                p!(error);
                panic!()
            }
            InferErrorVariant::Original { ref message, range } => Self {
                severity: DiagnosticSeverity::Error,
                range: range.clone(),
                message: format!("Infer Error: {}", message),
                dev_src: error.dev_src.clone(),
            },
        }
    }
}

impl From<&LexError> for Diagnostic {
    fn from(error: &LexError) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            range: error.range.clone(),
            message: format!("Lex Error: {}", error.message),
            dev_src: error.dev_src.clone(),
        }
    }
}

impl From<EntitySyntaxError> for Diagnostic {
    fn from(e: EntitySyntaxError) -> Self {
        Diagnostic {
            severity: DiagnosticSeverity::Error,
            range: match e.kind {
                EntitySyntaxErrorKind::Defn { range } => range,
                _ => Default::default(),
            },
            message: format!(
                "Entity Route Error: {}",
                &e.print_inherent(TestDisplayConfig {
                    colored: false,
                    indent: 0
                })
            ),
            dev_src: e.dev_src,
        }
    }
}

impl From<&EntitySyntaxError> for Diagnostic {
    fn from(e: &EntitySyntaxError) -> Self {
        Diagnostic {
            severity: DiagnosticSeverity::Error,
            range: match e.kind {
                EntitySyntaxErrorKind::Defn { range } => range,
                _ => Default::default(),
            },
            message: format!(
                "Entity Route Error: {}",
                &e.print_inherent(TestDisplayConfig {
                    colored: false,
                    indent: 0
                })
            ),
            dev_src: e.dev_src.clone(),
        }
    }
}

impl From<SemanticError> for Diagnostic {
    fn from(error: SemanticError) -> Self {
        match error.variant {
            SemanticErrorVariant::Derived { .. } => panic!(),
            SemanticErrorVariant::Original { message } => Self {
                severity: DiagnosticSeverity::Error,
                range: Default::default(),
                message,
                dev_src: error.dev_src,
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
