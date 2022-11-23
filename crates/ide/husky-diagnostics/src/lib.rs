mod collect;
mod db;
mod kind;
// mod reserve;
mod severity;

pub use db::DiagnosticsDb;
pub use kind::DiagnosticKind;
pub use severity::DiagnosticSeverity;

use collect::collect_module_diagnostics;
use husky_ast::{AstError, AstErrorVariant};
use husky_dev_utils::DevSource;
use husky_display_utils::{HuskyDisplay, HuskyDisplayConfig};
use husky_entity_tree::{EntityTreeError, EntityTreeErrorKind};
use husky_print_utils::p;
use husky_text::TextRange;
use husky_tokenize::LexError;
use std::fmt::Write;
use std::sync::Arc;

#[salsa::jar(db = DiagnosticsDb)]
pub struct DiagnosticsJar();

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Diagnostic {
    severity: DiagnosticSeverity,
    range: TextRange,
    message: String,
    dev_src: DevSource,
}

impl HuskyDisplay for Diagnostic {
    fn write_inherent(&self, _config: HuskyDisplayConfig, result: &mut String) {
        write!(result, "{:?}\t{}", self.range, self.message).unwrap()
    }
}

// impl From<&AstError> for Diagnostic {
//     fn from(error: &AstError) -> Self {
//         match error.variant {
//             AstErrorVariant::Original { ref message, range } => Self {
//                 severity: DiagnosticSeverity::Error,
//                 range: range.clone(),
//                 message: format!("Ast Error: {}", message),
//                 dev_src: error.dev_src.clone(),
//             },
//             AstErrorVariant::Derived => panic!(),
//         }
//     }
// }

// impl From<&InferError> for Diagnostic {
//     fn from(error: &InferError) -> Self {
//         match error.variant {
//             InferErrorVariant::Derived { .. } => {
//                 p!(error);
//                 panic!()
//             }
//             InferErrorVariant::Original { ref message, range } => Self {
//                 severity: DiagnosticSeverity::Error,
//                 range: range.clone(),
//                 message: format!("Infer Error: {}", message),
//                 dev_src: error.dev_src.clone(),
//             },
//         }
//     }
// }

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

impl From<EntityTreeError> for Diagnostic {
    fn from(e: EntityTreeError) -> Self {
        Diagnostic {
            severity: DiagnosticSeverity::Error,
            range: match e.kind {
                EntityTreeErrorKind::Defn { range } => range,
                _ => Default::default(),
            },
            message: format!(
                "Entity Route Error: {}",
                &e.print_inherent(HuskyDisplayConfig {
                    colored: false,
                    indent: 0
                })
            ),
            dev_src: e.dev_src,
        }
    }
}

impl From<&EntityTreeError> for Diagnostic {
    fn from(e: &EntityTreeError) -> Self {
        Diagnostic {
            severity: DiagnosticSeverity::Error,
            range: match e.kind {
                EntityTreeErrorKind::Defn { range } => range,
                _ => Default::default(),
            },
            message: format!(
                "Entity Route Error: {}",
                &e.print_inherent(HuskyDisplayConfig {
                    colored: false,
                    indent: 0
                })
            ),
            dev_src: e.dev_src.clone(),
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
