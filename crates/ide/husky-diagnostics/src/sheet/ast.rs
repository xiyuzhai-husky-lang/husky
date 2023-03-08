use husky_ast::{Ast, AstError, OriginalAstError};
use husky_token::TokenGroupIdx;

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
    let ctx = SheetDiagnosticsContext::new(db, module_path);
    if let (Ok(ranged_token_sheet), Ok(ast_sheet)) = (
        db.ranged_token_sheet(module_path),
        db.ast_sheet(module_path),
    ) {
        let _token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        for ast in ast_sheet.data() {
            match ast {
                Ast::Err {
                    token_group_idx,
                    error: AstError::Original(error),
                } => diagnostics.push((*token_group_idx, error).to_diagnostic(&ctx)),
                _ => (),
            }
        }
    }
    // todo
    AstDiagnosticSheet::new(db, diagnostics)
}
impl Diagnose for (TokenGroupIdx, &OriginalAstError) {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, _db: &SheetDiagnosticsContext) -> String {
        match self.1 {
            OriginalAstError::ExcessiveIndent => format!("Syntax Error: excessive indent"),
            OriginalAstError::StandaloneElif => format!("Syntax Error: standalone elif"),
            OriginalAstError::StandaloneElse => format!("Syntax Error: standalone else"),
            OriginalAstError::ExpectEntityKeyword => {
                format!("Syntax Error: expected entity keyword")
            }
            OriginalAstError::ExpectDecoratorOrEntityKeyword => {
                format!("Syntax Error: expected decorator or entity keyword")
            }
            OriginalAstError::ExpectIdent(_) => format!("Syntax Error: expected identifier"),
            OriginalAstError::UnexpectedEndOfTokenGroupAfterPubKeyword(_) => {
                format!("Syntax Error: unexpected end after `pub`")
            }
            OriginalAstError::ExpectNothing => format!("Syntax Error: expected nothing"),
            OriginalAstError::UnexpectedStmtInsideModule => {
                format!("Syntax Error: unexpected stmt inside module")
            }
            OriginalAstError::UnexpectedStmtInsideImpl => {
                format!("Syntax Error: unexpected stmt inside impl")
            }
            OriginalAstError::UnexpectedTokenForTraitItem(_) => {
                format!("Syntax Error: unexpected token for trait item")
            }
            OriginalAstError::UnexpectedTokenForTypeImplItem(_) => {
                format!("Syntax Error: unexpected token for type implementation item")
            }
            OriginalAstError::UnexpectedTokenForTraitImplItem(_) => {
                format!("Syntax Error: unexpected token for trait implementation item")
            }
            OriginalAstError::UnexpectedTokenForModuleItem(_) => {
                format!("Syntax Error: unexpected token for module item")
            }
            OriginalAstError::InvalidAstForDefinitionOrUse => {
                format!("Syntax Error: invalid ast for definition or use")
            }
            OriginalAstError::Todo => {
                format!("Syntax Error: ast error todo")
            }
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &SheetDiagnosticsContext) -> TextRange {
        // merge branches
        match self.1 {
            OriginalAstError::ExcessiveIndent
            | OriginalAstError::StandaloneElif
            | OriginalAstError::StandaloneElse
            | OriginalAstError::ExpectEntityKeyword
            | OriginalAstError::ExpectDecoratorOrEntityKeyword
            | OriginalAstError::ExpectNothing
            | OriginalAstError::UnexpectedStmtInsideModule
            | OriginalAstError::UnexpectedStmtInsideImpl
            | OriginalAstError::InvalidAstForDefinitionOrUse
            | OriginalAstError::Todo => {
                let token_idx_range = ctx.token_sheet_data().token_group_token_idx_range(self.0);
                ctx.ranged_token_sheet().tokens_text_range(token_idx_range)
            }
            OriginalAstError::ExpectIdent(token_idx)
            | OriginalAstError::UnexpectedEndOfTokenGroupAfterPubKeyword(token_idx)
            | OriginalAstError::UnexpectedTokenForTraitItem(token_idx)
            | OriginalAstError::UnexpectedTokenForTypeImplItem(token_idx)
            | OriginalAstError::UnexpectedTokenForTraitImplItem(token_idx)
            | OriginalAstError::UnexpectedTokenForModuleItem(token_idx) => {
                ctx.ranged_token_sheet().token_text_range(*token_idx)
            }
        }
    }
}
