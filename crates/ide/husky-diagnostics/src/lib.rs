mod collect;
mod db;
mod kind;
// mod reserve;
mod severity;

pub use db::DiagnosticsDb;
use husky_token::TokenError;
pub use kind::DiagnosticKind;
pub use severity::DiagnosticSeverity;

use collect::collect_module_diagnostics;
use husky_ast::{AstError, AstErrorVariant};
use husky_dev_utils::DevSource;
use husky_display_utils::{HuskyDisplay, HuskyDisplayConfig};
use husky_print_utils::p;
use husky_text::TextRange;
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
