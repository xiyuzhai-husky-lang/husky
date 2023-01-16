use std::fmt::format;

use husky_token::TokenError;

use super::*;

#[salsa::tracked(jar = DiagnosticsJar)]
pub struct TokenDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn token_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> TokenDiagnosticSheet {
    let mut diagnostics = vec![];
    // todo
    TokenDiagnosticSheet::new(db, diagnostics)
}

fn token_error_message(error: &TokenError) -> String {
    match error {
        TokenError::IncompleteStringLiteral => format!("Syntax Error: incomplete string literal"),
        TokenError::UnexpectedCharAfterBackslash => {
            format!("Syntax Error: unexpected char after backslash")
        }
        TokenError::UnrecognizedChar(_) => format!("Syntax Error: unrecognized char"),
        TokenError::IllFormedLiteral(_) => format!("Syntax Error: ill-formed literal"),
        TokenError::NumberPseudoLiteral(_) => format!("Syntax Error: number pseudoliteral"),
        TokenError::ParseIntError => format!("Syntax Error: parse int error"),
        TokenError::InvalidIntegerSuffix => format!("Syntax Error: invalid integer suffix"),
        TokenError::InvalidIdentifier => format!("Syntax Error: invalid identifier"),
    }
}
