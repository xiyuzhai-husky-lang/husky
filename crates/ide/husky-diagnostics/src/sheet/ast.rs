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
        AstError::StandaloneElif => format!("Syntax Error:standalone elif"),
        AstError::StandaloneElse => format!("Syntax Error: standalone else"),
        AstError::ExpectEntityKeyword => format!("Syntax Error: expect entity keyword"),
        AstError::ExpectDecoratorOrEntityKeyword => {
            format!("Syntax Error:expect decorator or entity keyword")
        }
        AstError::ExpectIdentifier(_) => format!("Syntax Error: expect identifier"),
        AstError::ExpectParBraOrDecoratorOrIdentifier(_) => {
            format!("Syntax Error: expect `(` or decorator or identifier")
        }
        AstError::ExpectNothing => format!("Syntax Error: expect nothing"),
        AstError::Token(_) => format!("Syntax Error: token error"),
        AstError::UnexpectedStmtInsideModule => {
            format!("Syntax Error: unexpect stmt inside module")
        }
        AstError::UnexpectedStmtInsideImpl => format!("Syntax Error: unexpect stmt inside impl"),
    }
}
