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
        TokenError::IncompleteStringLiteral => format!("Syntax Error: mismatching bracket"),
        TokenError::UnexpectedCharAfterBackslash => todo!(),
        TokenError::UnrecognizedChar(_) => todo!(),
        TokenError::IllFormedLiteral(_) => todo!(),
        TokenError::NumberPseudoLiteral(_) => todo!(),
        TokenError::ParseIntError => todo!(),
    }
}
