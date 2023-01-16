use husky_ast::AstError;

use super::*;

#[salsa::tracked(jar = DiagnosticsJar)]
pub struct AstDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}
// ad hoc
impl<Db: DiagnosticsDb> salsa::DebugWithDb<Db> for AstDiagnosticSheet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn DiagnosticsDb, include_all_fields)
    }
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn ast_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> AstDiagnosticSheet {
    let mut diagnostics = vec![];
    // todo
    AstDiagnosticSheet::new(db, diagnostics)
}

fn ast_error_message(error: &AstError) -> String {
    match error {
        AstError::ExcessiveIndent => format!("Syntax Error: excessive indent"),
        AstError::StandaloneElif => format!("Syntax Error: standalone elif"),
        AstError::StandaloneElse => format!("Syntax Error: standalone else"),
        AstError::ExpectEntityKeyword => format!("Syntax Error: expected entity keyword"),
        AstError::ExpectDecoratorOrEntityKeyword => {
            format!("Syntax Error: expected decorator or entity keyword")
        }
        AstError::ExpectIdentifier(_) => format!("Syntax Error: expected identifier"),
        AstError::ExpectParBraOrDecoratorOrIdentifier(_) => {
            format!("Syntax Error: expected `(` or decorator or identifier")
        }
        AstError::ExpectNothing => format!("Syntax Error: expected nothing"),
        AstError::Token(_) => format!("Syntax Error: token error"),
        AstError::UnexpectedStmtInsideModule => {
            format!("Syntax Error: unexpected stmt inside module")
        }
        AstError::UnexpectedStmtInsideImpl => format!("Syntax Error: unexpected stmt inside impl"),
        AstError::UnexpectedTokenForTraitItem(_) => {
            format!("Syntax Error: unexpected token for trait item")
        }
        AstError::UnexpectedTokenForTypeImplItem(_) => {
            format!("Syntax Error: unexpected token for type implementation item")
        }
    }
}
