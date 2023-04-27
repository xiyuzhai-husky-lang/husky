use husky_ast::{Ast, AstError, OriginalAstError};
use husky_scope_expr::OriginalVisibilityExprError;
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
            OriginalAstError::ExpectedEntityKeyword => {
                format!("Syntax Error: expected entity keyword")
            }
            OriginalAstError::ExpectedDecoratorOrEntityKeyword => {
                format!("Syntax Error: expected decorator or entity keyword")
            }
            OriginalAstError::ExpectedIdent(_) => format!("Syntax Error: expected identifier"),
            OriginalAstError::UnexpectedEndOfTokenGroupAfterPubKeyword(_) => {
                format!("Syntax Error: unexpected end after `pub`")
            }
            OriginalAstError::ExpectNothing => format!("Syntax Error: expected nothing"),
            OriginalAstError::UnexpectedStmtInsideImplBlock => {
                format!("Syntax Error: unexpected stmt inside impl")
            }
            OriginalAstError::UnexpectedPunctuationForTraitItem(_, unexpected_punctuation) => {
                format!("Syntax Error: unexpected punctuation `{unexpected_punctuation}` for trait item")
            }
            OriginalAstError::UnexpectedTokenForTraitItem(_) => {
                format!("Syntax Error: unexpected token for trait item")
            }
            OriginalAstError::UnexpectedPunctuationForTypeImplItem(_, unexpected_punctuation) => {
                format!("Syntax Error: unexpected punctuation `{unexpected_punctuation}` for type implementation item")
            }
            OriginalAstError::UnexpectedTokenForTypeImplItem(_) => {
                format!("Syntax Error: unexpected token for type implementation item")
            }
            OriginalAstError::UnexpectedPunctuationForTraitForTypeImplItem(
                _,
                unexpected_punctuation,
            ) => {
                format!("Syntax Error: unexpected punctuation `{unexpected_punctuation}` for trait implementation item")
            }
            OriginalAstError::UnexpectedTokenForTraitForTypeImplItem(_) => {
                format!("Syntax Error: unexpected token for trait implementation item")
            }
            OriginalAstError::UnexpectedPunctuationForConnectedModuleItem(
                _,
                unexpected_punctuation,
            ) => {
                format!("Syntax Error: unexpected punctuation `{unexpected_punctuation}` for connected module item")
            }
            OriginalAstError::UnexpectedTokenForConnectedModuleItem(_) => {
                format!("Syntax Error: unexpected token for connected module item")
            }
            OriginalAstError::UnexpectedPunctuationForDisconnectedModuleItem(
                _,
                unexpected_punctuation,
            ) => {
                format!("Syntax Error: unexpected punctuation `{unexpected_punctuation}` for disconnected module item")
            }
            OriginalAstError::UnexpectedTokenForDisconnectedModuleItem(_) => {
                format!("Syntax Error: unexpected token for disconnected module item")
            }
            OriginalAstError::InvalidAstForDefinitionOrUse => {
                format!("Syntax Error: invalid ast for definition or use")
            }
            OriginalAstError::Todo => {
                format!("Syntax Error: ast error todo")
            }
            OriginalAstError::UnexpectedEndAfterFormKeywordInsideModule => {
                format!("Syntax Error: UnexpectedEndAfterFormKeywordInsideModule")
            }
            OriginalAstError::UnexpectedEndAfterFormKeywordInsideTrait => {
                format!("Syntax Error: UnexpectedEndAfterFormKeywordInsideTrait")
            }
            OriginalAstError::UnexpectedEndAfterFormKeywordInsideTypeImplBlock => {
                format!("Syntax Error: UnexpectedEndAfterFormKeywordInsideTypeImplBlock")
            }
            OriginalAstError::UnexpectedEndAfterFormKeywordInsideTraitForTypeImplBlock => {
                format!("Syntax Error: UnexpectedEndAfterFormKeywordInsideTraitForTypeImplBlock")
            }
            OriginalAstError::UnexpectedStmtInsideTrait => {
                format!("Syntax Error: UnexpectedStmtInsideTrait")
            }
            OriginalAstError::UnexpectedMainInsideTrait => {
                format!("Syntax Error: UnexpectedMainInsideTrait")
            }
            OriginalAstError::UnexpectedUseInsideTrait => {
                format!("Syntax Error: UnexpectedUseInsideTrait")
            }
            OriginalAstError::UnexpectedModInsideTrait => {
                format!("Syntax Error: UnexpectedModInsideTrait")
            }
            OriginalAstError::UnexpectedVisualInsideTrait => {
                format!("Syntax Error: UnexpectedVisualInsideTrait")
            }
            OriginalAstError::UnexpectedImplInsideTrait => {
                format!("Syntax Error: UnexpectedImplInsideTrait")
            }
            OriginalAstError::UnexpectedTraitInsideTrait => {
                format!("Syntax Error: UnexpectedTraitInsideTrait")
            }
            OriginalAstError::UnexpectedPattern => {
                format!("Syntax Error: UnexpectedPattern")
            }
            OriginalAstError::UnexpectedModInsideForm => {
                format!("Syntax Error: UnexpectedModInsideForm")
            }
            OriginalAstError::UnexpectedVisualInsideForm => {
                format!("Syntax Error: UnexpectedVisualInsideForm")
            }
            OriginalAstError::UnexpectedImplInsideForm => {
                format!("Syntax Error: UnexpectedImplInsideForm")
            }
            OriginalAstError::UnexpectedTraitInsideForm => {
                format!("Syntax Error: UnexpectedTraitInsideForm")
            }
            OriginalAstError::UnexpectedEndKeywordAsFirstNonCommentToken => {
                format!("Syntax Error: UnexpectedEndKeyword")
            }
            OriginalAstError::UnexpectedTypeDefnInsideTypeImplBlock => {
                format!("Syntax Error: UnexpectedTypeDefnInsideTypeImplBlock")
            }
            OriginalAstError::ExpectedEntityKeywordGroup(_) => {
                format!("Syntax Error: ExpectedEntityKeywordGroup")
            }
            OriginalAstError::VisibilityExprError(_) => {
                format!("Syntax Error: VisibilityExprError")
            }
            OriginalAstError::UnexpectedMemoFieldInsideForm => {
                format!("Syntax Error: UnexpectedMemoFieldInsideForm")
            }
            OriginalAstError::UnexpectedStmtInsideModule => {
                format!("Syntax Error: UnexpectedStmtInsideModule")
            }
            OriginalAstError::ExpectedTraitForTypeItems(_) => {
                format!("Syntax Error: ExpectedTraitForTypeItems")
            }
            OriginalAstError::ExpectedTypeItems(_) => {
                format!("Syntax Error: ExpectedTypeItems")
            }
            OriginalAstError::ExpectedTypeVariants(_) => {
                format!("Syntax Error: ExpectedTypeVariants")
            }
            OriginalAstError::ExpectedIdentForTypeVariant(_) => {
                format!("Syntax Error: ExpectedIdentForTypeVariant")
            }
            OriginalAstError::ExpectedFormBodyForConfig(_) => {
                format!("Syntax Error: ExpectedFormBodyForConfig")
            }
            OriginalAstError::ExpectedFormBodyForMain(_) => {
                format!("Syntax Error: ExpectedFormBodyForMain")
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
            | OriginalAstError::ExpectedEntityKeyword
            | OriginalAstError::ExpectedDecoratorOrEntityKeyword
            | OriginalAstError::ExpectNothing
            | OriginalAstError::UnexpectedStmtInsideImplBlock
            | OriginalAstError::InvalidAstForDefinitionOrUse
            | OriginalAstError::Todo
            | OriginalAstError::UnexpectedEndAfterFormKeywordInsideModule
            | OriginalAstError::UnexpectedEndAfterFormKeywordInsideTrait
            | OriginalAstError::UnexpectedEndAfterFormKeywordInsideTypeImplBlock
            | OriginalAstError::UnexpectedEndAfterFormKeywordInsideTraitForTypeImplBlock
            | OriginalAstError::UnexpectedStmtInsideTrait
            | OriginalAstError::UnexpectedMainInsideTrait
            | OriginalAstError::UnexpectedUseInsideTrait
            | OriginalAstError::UnexpectedModInsideTrait
            | OriginalAstError::UnexpectedVisualInsideTrait
            | OriginalAstError::UnexpectedImplInsideTrait
            | OriginalAstError::UnexpectedTraitInsideTrait
            | OriginalAstError::UnexpectedPattern
            | OriginalAstError::UnexpectedModInsideForm
            | OriginalAstError::UnexpectedVisualInsideForm
            | OriginalAstError::UnexpectedImplInsideForm
            | OriginalAstError::UnexpectedTraitInsideForm
            | OriginalAstError::UnexpectedEndKeywordAsFirstNonCommentToken
            | OriginalAstError::UnexpectedTypeDefnInsideTypeImplBlock
            | OriginalAstError::ExpectedEntityKeywordGroup(_)
            | OriginalAstError::UnexpectedMemoFieldInsideForm
            | OriginalAstError::UnexpectedStmtInsideModule
            | OriginalAstError::ExpectedTraitForTypeItems(_)
            | OriginalAstError::ExpectedTypeItems(_)
            | OriginalAstError::ExpectedTypeVariants(_)
            | OriginalAstError::ExpectedIdentForTypeVariant(_)
            | OriginalAstError::ExpectedFormBodyForConfig(_)
            | OriginalAstError::ExpectedFormBodyForMain(_) => ctx.token_group_text_range(self.0),
            OriginalAstError::ExpectedIdent(token_idx)
            | OriginalAstError::UnexpectedEndOfTokenGroupAfterPubKeyword(token_idx)
            | OriginalAstError::UnexpectedTokenForTraitItem(token_idx)
            | OriginalAstError::UnexpectedPunctuationForTraitItem(token_idx, _)
            | OriginalAstError::UnexpectedTokenForTypeImplItem(token_idx)
            | OriginalAstError::UnexpectedPunctuationForTypeImplItem(token_idx, _)
            | OriginalAstError::UnexpectedTokenForTraitForTypeImplItem(token_idx)
            | OriginalAstError::UnexpectedPunctuationForTraitForTypeImplItem(token_idx, _)
            | OriginalAstError::UnexpectedTokenForConnectedModuleItem(token_idx)
            | OriginalAstError::UnexpectedPunctuationForConnectedModuleItem(token_idx, _)
            | OriginalAstError::UnexpectedTokenForDisconnectedModuleItem(token_idx)
            | OriginalAstError::UnexpectedPunctuationForDisconnectedModuleItem(token_idx, _)
            | OriginalAstError::VisibilityExprError(
                OriginalVisibilityExprError::NoSuperForRoot(token_idx)
                | OriginalVisibilityExprError::ExpectRightParenthesis(token_idx)
                | OriginalVisibilityExprError::ExpectCrateOrSuper(token_idx),
            ) => ctx.token_text_range(*token_idx),
        }
    }
}
