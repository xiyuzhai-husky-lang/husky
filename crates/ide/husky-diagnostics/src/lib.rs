mod collect;
mod db;
mod severity;
mod sheet;
#[cfg(test)]
mod tests;

pub use db::DiagnosticsDb;
pub use severity::DiagnosticSeverity;
pub use sheet::*;
#[cfg(test)]
pub use tests::*;

use collect::collect_module_diagnostics;
use husky_dev_utils::DevSource;
use husky_text::TextRange;
use husky_vfs::*;
use std::fmt::Write;
use std::sync::Arc;

#[salsa::jar(db = DiagnosticsDb)]
pub struct DiagnosticsJar(
    DiagnosticSheet,
    diagnostic_sheet,
    EntityTreeDiagnosticSheet,
    entity_tree_diagnostic_sheet,
    AstDiagnosticSheet,
    ast_diagnostic_sheet,
    TokenDiagnosticSheet,
    token_diagnostic_sheet,
    ExprDiagnosticSheet,
    expr_diagnostic_sheet,
);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Diagnostic {
    severity: DiagnosticSeverity,
    range: TextRange,
    message: String,
    dev_src: DevSource,
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

impl Into<lsp_types::Diagnostic> for &Diagnostic {
    fn into(self) -> lsp_types::Diagnostic {
        lsp_types::Diagnostic {
            range: self.range.into(),
            severity: Some(self.severity.into()),
            code: None,
            code_description: None,
            source: Some("husky-analyzer".to_string()),
            message: self.message.clone(),
            related_information: None,
            tags: None,
            data: None,
        }
    }
}
