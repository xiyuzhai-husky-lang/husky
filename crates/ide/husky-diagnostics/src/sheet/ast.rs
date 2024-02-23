use super::*;
use husky_ast::HasAstSheet;
use husky_ast::{
    error::{AstError, OriginalAstError},
    AstData,
};
use husky_scope_expr::OriginalVisibilityExprError;
use husky_token::verse::idx::TokenVerseIdx;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct AstDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn ast_diagnostic_sheet(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> AstDiagnosticSheet {
    let mut diagnostics = vec![];
    let ctx = SheetDiagnosticsContext::new(db, module_path);
    for ast in module_path.ast_sheet(db).data() {
        match ast {
            AstData::Err {
                token_verse_idx,
                error: AstError::Original(error),
            } => diagnostics.push((*token_verse_idx, error).to_diagnostic(&ctx)),
            _ => (),
        }
    }
    // todo
    AstDiagnosticSheet::new(db, diagnostics)
}
impl Diagnose for (TokenVerseIdx, &OriginalAstError) {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, _db: &SheetDiagnosticsContext) -> String {
        match self.1 {
            OriginalAstError::ExcessiveIndent => format!("Syntax Error: excessive indent"),
            OriginalAstError::StandaloneElif => format!("Syntax Error: standalone elif"),
            OriginalAstError::StandaloneElse => format!("Syntax Error: standalone else"),
            OriginalAstError::ExpectedEntityKeyword => {
                format!("Syntax Error: expected item keyword")
            }
            OriginalAstError::ExpectedDecoratorOrEntityKeyword => {
                format!("Syntax Error: expected decorator or item keyword")
            }
            OriginalAstError::ExpectedIdent(_) => format!("Syntax Error: expected identifier"),
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
            OriginalAstError::UnexpectedPunctuationForConnectedMajorItem(
                _,
                unexpected_punctuation,
            ) => {
                format!("Syntax Error: unexpected punctuation `{unexpected_punctuation}` for connected module item")
            }
            OriginalAstError::UnexpectedTokenForConnectedMajorItem(_) => {
                format!("Syntax Error: unexpected token for connected module item")
            }
            OriginalAstError::UnexpectedPunctuationForDisconnectedMajorItem(
                _,
                unexpected_punctuation,
            ) => {
                format!("Syntax Error: unexpected punctuation `{unexpected_punctuation}` for disconnected module item")
            }
            OriginalAstError::UnexpectedTokenForDisconnectedMajorItem(_) => {
                format!("Syntax Error: unexpected token for disconnected module item")
            }
            OriginalAstError::InvalidAstForDefinitionOrUse => {
                format!("Syntax Error: invalid ast for definition or use")
            }
            OriginalAstError::Todo => {
                format!("Syntax Error: ast error todo")
            }
            OriginalAstError::UnexpectedEndAfterFugitiveKeywordInsideModule => {
                format!("Syntax Error: UnexpectedEndAfterFugitiveKeywordInsideModule")
            }
            OriginalAstError::UnexpectedEndAfterFugitiveKeywordInsideTrait => {
                format!("Syntax Error: UnexpectedEndAfterFugitiveKeywordInsideTrait")
            }
            OriginalAstError::UnexpectedEndAfterFugitiveKeywordInsideTypeImplBlock => {
                format!("Syntax Error: UnexpectedEndAfterFugitiveKeywordInsideTypeImplBlock")
            }
            OriginalAstError::UnexpectedEndAfterFugitiveKeywordInsideTraitForTypeImplBlock => {
                format!(
                    "Syntax Error: UnexpectedEndAfterFugitiveKeywordInsideTraitForTypeImplBlock"
                )
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
            OriginalAstError::UnexpectedImplBlockInsideModuleItem => {
                format!("Syntax Error: unexpected implemention block inside module item")
            }
            OriginalAstError::UnexpectedTraitInsideTrait => {
                format!("Syntax Error: UnexpectedTraitInsideTrait")
            }
            OriginalAstError::UnexpectedPattern => {
                format!("Syntax Error: UnexpectedPattern")
            }
            OriginalAstError::UnexpectedStaticFnOutsideImplBlock => {
                format!("Syntax Error: UnexpectedStaticFnInsideForm")
            }
            OriginalAstError::UnexpectedTraitInsideForm => {
                format!("Syntax Error: UnexpectedTraitInsideForm")
            }
            OriginalAstError::UnexpectedEndKeywordAsFirstNonCommentToken => {
                format!("Syntax Error: UnexpectedEndKeyword")
            }
            OriginalAstError::UnexpectedConnectionKeywordAsFirstNonCommentToken => {
                format!("Syntax Error: unexpected connection keyword as first non-comment token")
            }
            OriginalAstError::UnexpectedMajorTypeInsideImplBlock => {
                format!("Syntax Error: UnexpectedTypeDefnInsideTypeImplBlock")
            }
            OriginalAstError::ExpectedEntityKeywordGroup(_) => {
                format!("Syntax Error: ExpectedEntityKeywordGroup")
            }
            OriginalAstError::VisibilityExprError(_) => {
                format!("Syntax Error: VisibilityExprError")
            }
            OriginalAstError::UnexpectedMemoFieldOutsideImplBlock => {
                format!("Syntax Error: UnexpectedMemoFieldInsideForm")
            }
            OriginalAstError::UnexpectedStmtUnderModule => {
                format!("Syntax Error: UnexpectedStmtInsideModule")
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
            OriginalAstError::UnexpectedModUnderFugitive => {
                format!("unexpected submodule inside module item")
            }
            OriginalAstError::SubmoduleFileNotFound {
                ident_token: _,
                error: _,
            } => {
                format!("submodule file not found")
            }
            OriginalAstError::UnexpectedTraitInsideImplBlock => todo!(),
            OriginalAstError::ExpectedLboxOrIdentAfterPoundForAttrOrSorce => todo!(),
            OriginalAstError::UnexpectedMemoUnderModule => todo!(),
            OriginalAstError::UnexpectedMemoUnderFugitive => todo!(),
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
            | OriginalAstError::UnexpectedEndAfterFugitiveKeywordInsideModule
            | OriginalAstError::UnexpectedEndAfterFugitiveKeywordInsideTrait
            | OriginalAstError::UnexpectedEndAfterFugitiveKeywordInsideTypeImplBlock
            | OriginalAstError::UnexpectedEndAfterFugitiveKeywordInsideTraitForTypeImplBlock
            | OriginalAstError::UnexpectedStmtInsideTrait
            | OriginalAstError::UnexpectedMainInsideTrait
            | OriginalAstError::UnexpectedUseInsideTrait
            | OriginalAstError::UnexpectedModUnderFugitive
            | OriginalAstError::UnexpectedImplBlockInsideModuleItem
            | OriginalAstError::UnexpectedTraitInsideTrait
            | OriginalAstError::UnexpectedPattern
            | OriginalAstError::UnexpectedStaticFnOutsideImplBlock
            | OriginalAstError::UnexpectedTraitInsideForm
            | OriginalAstError::UnexpectedEndKeywordAsFirstNonCommentToken
            | OriginalAstError::UnexpectedConnectionKeywordAsFirstNonCommentToken
            | OriginalAstError::UnexpectedMajorTypeInsideImplBlock
            | OriginalAstError::ExpectedEntityKeywordGroup(_)
            | OriginalAstError::UnexpectedMemoFieldOutsideImplBlock
            | OriginalAstError::UnexpectedStmtUnderModule
            | OriginalAstError::ExpectedTypeItems(_)
            | OriginalAstError::ExpectedTypeVariants(_)
            | OriginalAstError::ExpectedIdentForTypeVariant(_)
            | OriginalAstError::ExpectedFormBodyForConfig(_)
            | OriginalAstError::ExpectedFormBodyForMain(_) => ctx.token_verse_text_range(self.0),
            OriginalAstError::ExpectedIdent(token_stream_state)
            | OriginalAstError::VisibilityExprError(
                OriginalVisibilityExprError::ExpectedRightParenthesis(token_stream_state)
                | OriginalVisibilityExprError::ExpectedCrateOrSuper(token_stream_state),
            ) => ctx.token_stream_state_text_range(*token_stream_state),
            OriginalAstError::UnexpectedTokenForTraitItem(token_idx)
            | OriginalAstError::UnexpectedPunctuationForTraitItem(token_idx, _)
            | OriginalAstError::UnexpectedTokenForTypeImplItem(token_idx)
            | OriginalAstError::UnexpectedPunctuationForTypeImplItem(token_idx, _)
            | OriginalAstError::UnexpectedTokenForTraitForTypeImplItem(token_idx)
            | OriginalAstError::UnexpectedPunctuationForTraitForTypeImplItem(token_idx, _)
            | OriginalAstError::UnexpectedTokenForConnectedMajorItem(token_idx)
            | OriginalAstError::UnexpectedPunctuationForConnectedMajorItem(token_idx, _)
            | OriginalAstError::UnexpectedTokenForDisconnectedMajorItem(token_idx)
            | OriginalAstError::UnexpectedPunctuationForDisconnectedMajorItem(token_idx, _)
            | OriginalAstError::VisibilityExprError(OriginalVisibilityExprError::NoSuperForRoot(
                token_idx,
            )) => ctx.token_idx_text_range(*token_idx),
            OriginalAstError::SubmoduleFileNotFound { ident_token, .. } => {
                ctx.token_idx_text_range(ident_token.token_idx())
            }
            OriginalAstError::UnexpectedTraitInsideImplBlock => todo!(),
            OriginalAstError::ExpectedLboxOrIdentAfterPoundForAttrOrSorce => todo!(),
            OriginalAstError::UnexpectedMemoUnderModule => todo!(),
            OriginalAstError::UnexpectedMemoUnderFugitive => todo!(),
        }
    }
}
