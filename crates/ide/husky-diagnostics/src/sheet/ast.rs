use husky_ast::{Ast, AstError};
use husky_token::{TokenGroupIdx, TokenSheetData};

use super::*;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct AstDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn ast_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> AstDiagnosticSheet {
    let mut diagnostics = vec![];
    let ctx = DiagnosticsSheetContext::new(db, module_path);
    if let (Ok(ranged_token_sheet), Ok(ast_sheet)) = (
        db.ranged_token_sheet(module_path),
        db.ast_sheet(module_path),
    ) {
        let token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        for ast in ast_sheet.data() {
            match ast {
                Ast::Err {
                    token_group_idx,
                    error,
                } => diagnostics.push((*token_group_idx, error).to_diagnostic(&ctx)),
                _ => (),
            }
        }
    }
    // todo
    AstDiagnosticSheet::new(db, diagnostics)
}
impl Diagnose for (TokenGroupIdx, &AstError) {
    type Context<'a> = DiagnosticsSheetContext<'a>;

    fn message(&self, db: &DiagnosticsSheetContext) -> String {
        match self.1 {
            AstError::ExcessiveIndent => format!("Syntax Error: excessive indent"),
            AstError::StandaloneElif => format!("Syntax Error: standalone elif"),
            AstError::StandaloneElse => format!("Syntax Error: standalone else"),
            AstError::ExpectEntityKeyword => format!("Syntax Error: expected entity keyword"),
            AstError::ExpectDecoratorOrEntityKeyword => {
                format!("Syntax Error: expected decorator or entity keyword")
            }
            AstError::ExpectIdentifier(_) => format!("Syntax Error: expected identifier"),
            AstError::UnexpectedEndOfTokenGroupAfterPubKeyword(_) => {
                format!("Syntax Error: unexpected end after `pub`")
            }
            AstError::ExpectNothing => format!("Syntax Error: expected nothing"),
            AstError::UnexpectedStmtInsideModule => {
                format!("Syntax Error: unexpected stmt inside module")
            }
            AstError::UnexpectedStmtInsideImpl => {
                format!("Syntax Error: unexpected stmt inside impl")
            }
            AstError::UnexpectedTokenForTraitItem(_) => {
                format!("Syntax Error: unexpected token for trait item")
            }
            AstError::UnexpectedTokenForTypeImplItem(_) => {
                format!("Syntax Error: unexpected token for type implementation item")
            }
            AstError::UnexpectedTokenForTraitImplItem(_) => {
                format!("Syntax Error: unexpected token for trait implementation item")
            }
            AstError::UnexpectedTokenForModuleItem(_) => {
                format!("Syntax Error: unexpected token for module item")
            }
            AstError::InvalidAstForDefinitionOrUse => {
                format!("Syntax Error: invalid ast for definition or use")
            }
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &DiagnosticsSheetContext) -> TextRange {
        // merge branches
        match self.1 {
            AstError::ExcessiveIndent
            | AstError::StandaloneElif
            | AstError::StandaloneElse
            | AstError::ExpectEntityKeyword
            | AstError::ExpectDecoratorOrEntityKeyword
            | AstError::ExpectNothing
            | AstError::UnexpectedStmtInsideModule
            | AstError::UnexpectedStmtInsideImpl
            | AstError::InvalidAstForDefinitionOrUse => {
                let token_idx_range = ctx.token_sheet_data().token_group_token_idx_range(self.0);
                ctx.ranged_token_sheet().tokens_text_range(token_idx_range)
            }
            AstError::ExpectIdentifier(token_idx)
            | AstError::UnexpectedEndOfTokenGroupAfterPubKeyword(token_idx)
            | AstError::UnexpectedTokenForTraitItem(token_idx)
            | AstError::UnexpectedTokenForTypeImplItem(token_idx)
            | AstError::UnexpectedTokenForTraitImplItem(token_idx)
            | AstError::UnexpectedTokenForModuleItem(token_idx) => {
                ctx.ranged_token_sheet().token_text_range(*token_idx)
            }
        }
    }
}
