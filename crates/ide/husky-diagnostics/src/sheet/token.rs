use husky_token::{Token, TokenError};

use super::*;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct TokenDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn token_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> TokenDiagnosticSheet {
    let ctx = SheetDiagnosticsContext::new(db, module_path);
    let mut diagnostics = vec![];
    if let Ok(ranged_token_sheet) = db.ranged_token_sheet(module_path) {
        for (range, token) in ranged_token_sheet.ranged_token_iter(db) {
            if let Token::Error(e) = token {
                diagnostics.push((range, e).to_diagnostic(&ctx))
            }
        }
    }
    TokenDiagnosticSheet::new(db, diagnostics)
}

impl Diagnose for (&TextRange, &TokenError) {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, _db: &Self::Context<'_>) -> String {
        match self.1 {
            TokenError::IncompleteStringLiteralBeforeEof => {
                format!("Syntax Error: incomplete string literal before end of file")
            }
            TokenError::IncompleteStringLiteralBeforeEol => {
                format!("Syntax Error: incomplete string literal before end of line")
            }
            TokenError::UnexpectedCharAfterBackslash(c) => {
                format!("Syntax Error: unexpected char `{c}` after backslash")
            }
            TokenError::UnrecognizedChar(_) => format!("Syntax Error: unrecognized char"),
            TokenError::IllFormedLiteral(_) => format!("Syntax Error: ill-formed literal"),
            TokenError::NumberPseudoLiteral(_) => format!("Syntax Error: number pseudoliteral"),
            TokenError::ParseIntError => format!("Syntax Error: parse int error"),
            TokenError::InvalidIntegerSuffix => format!("Syntax Error: invalid integer suffix"),
            TokenError::InvalidIdent => format!("Syntax Error: invalid identifier"),
            TokenError::NothingAfterSingleQuote => format!("Syntax Error: nothing after `'`"),
            TokenError::InvalidLabel => format!("Syntax Error: InvalidLabel"),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, _ctx: &Self::Context<'_>) -> TextRange {
        // todo: modify for special cases like TokenError::IncompleteStringLiteralBeforeEol
        *self.0
    }
}
