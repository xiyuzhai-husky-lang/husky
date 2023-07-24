#![feature(trait_upcasting)]
mod collector;
mod context;
mod db;
mod severity;
mod sheet;
#[cfg(test)]
mod tests;

pub use db::DiagnosticsDb;
use husky_token::{RangedTokenSheet, TokenSheetData};
pub use severity::DiagnosticSeverity;

use collector::{ModuleDiagnosticsCollector, RegionDiagnosticsCollector};
use context::*;
use husky_text::TextRange;
use husky_vfs::*;
use sheet::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = DiagnosticsDb)]
pub struct DiagnosticsJar(
    DiagnosticSheet,
    diagnostic_sheet,
    EntityTreeDiagnosticSheet,
    item_tree_diagnostic_sheet,
    AstDiagnosticSheet,
    ast_diagnostic_sheet,
    TokenDiagnosticSheet,
    token_diagnostic_sheet,
    DeclDiagnosticSheet,
    decl_diagnostic_sheet,
    DefnDiagnosticSheet,
    defn_diagnostic_sheet,
    ExprDiagnosticSheet,
    expr_diagnostic_sheet,
    ExprTypeDiagnosticSheet,
    expr_ty_diagnostic_sheet,
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

trait Diagnose {
    type Context<'a>;

    fn message(&self, db: &Self::Context<'_>) -> String;
    fn severity(&self) -> DiagnosticSeverity;
    fn range(&self, ctx: &Self::Context<'_>) -> TextRange;
    fn to_diagnostic(&self, ctx: &Self::Context<'_>) -> Diagnostic {
        Diagnostic {
            message: self.message(ctx),
            severity: self.severity(),
            range: self.range(ctx),
        }
    }
}
