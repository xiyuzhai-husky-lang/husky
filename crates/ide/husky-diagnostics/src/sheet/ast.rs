use husky_ast::AstError;

use super::*;

#[salsa::tracked(jar = DiagnosticsJar)]
pub struct AstDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn ast_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> AstDiagnosticSheet {
    todo!()
}

fn ast_error_message(error: &AstError) {
    match error {
        AstError::ExcessiveIndent => todo!(),
        AstError::StandaloneElif => todo!(),
        AstError::StandaloneElse => todo!(),
        AstError::ExpectEntityKeyword => todo!(),
        AstError::ExpectDecoratorOrEntityKeyword => todo!(),
        AstError::ExpectIdentifier(_) => todo!(),
        AstError::ExpectParBraOrDecoratorOrIdentifier(_) => todo!(),
        AstError::ExpectNothing => todo!(),
        AstError::Token(_) => todo!(),
        AstError::UnexpectedStmtInsideModule => todo!(),
        AstError::UnexpectedStmtInsideImpl => todo!(),
    }
}
