#![feature(trait_upcasting)]
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
    message: String,
    severity: DiagnosticSeverity,
    range: TextRange,
}

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
