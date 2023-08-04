use husky_scope_expr::{
    DerivedVisibilityExprError, OriginalVisibilityExprError, VisibilityExprError,
};
use husky_token::{
    IdentToken, Punctuation, TokenError, TokenGroupIdx, TokenIdx, TokenIdxRange, TokenStreamParser,
    TokenStreamState,
};
use original_error::IntoError;
use thiserror::Error;

use crate::{AstDb, AstIdx};

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = AstDb)]
pub enum AstError {
    #[error("{0}")]
    Original(#[from] OriginalAstError),
    #[error("{0}")]
    Derived(#[from] DerivedAstError),
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = AstDb)]
pub enum OriginalAstError {
    #[error("excessive indent")]
    ExcessiveIndent,
    #[error("standalone elif")]
    StandaloneElif,
    #[error("standalone else")]
    StandaloneElse,
    #[error("expected item keyword")]
    ExpectedEntityKeyword,
    #[error("expected decorator or item keyword")]
    ExpectedDecoratorOrEntityKeyword,
    #[error("expected item keyword group")]
    ExpectedEntityKeywordGroup(TokenStreamState),
    #[error("expected identifier")]
    ExpectedIdent(TokenStreamState),
    #[error("expected nothing")]
    ExpectNothing,
    #[error("unexpected stmt inside impl")]
    UnexpectedStmtInsideImplBlock,
    #[error("unexpected punctuation `{1}` for trait item")]
    UnexpectedPunctuationForTraitItem(TokenIdx, Punctuation),
    #[error("unexpected token for trait item")]
    UnexpectedTokenForTraitItem(TokenIdx),
    #[error("unexpected punctuation `{1}` for type implementation item")]
    UnexpectedPunctuationForTypeImplItem(TokenIdx, Punctuation),
    #[error("unexpected token for type implementation item")]
    UnexpectedTokenForTypeImplItem(TokenIdx),
    #[error("unexpected punctuation `{1}` for type as trait implementation item")]
    UnexpectedPunctuationForTraitForTypeImplItem(TokenIdx, Punctuation),
    #[error("unexpected token for type as trait implementation item")]
    UnexpectedTokenForTraitForTypeImplItem(TokenIdx),
    #[error("unexpected punctuation `{1}` for connected module item")]
    UnexpectedPunctuationForConnectedMajorItem(TokenIdx, Punctuation),
    #[error("unexpected token for connected module item")]
    UnexpectedTokenForConnectedMajorItem(TokenIdx),
    #[error("unexpected punctuation `{1}` for disconnected module item")]
    UnexpectedPunctuationForDisconnectedMajorItem(TokenIdx, Punctuation),
    #[error("unexpected token for module item")]
    UnexpectedTokenForDisconnectedMajorItem(TokenIdx),
    #[error("invalid ast for definition or use")]
    InvalidAstForDefinitionOrUse,
    #[error("todo")]
    Todo,
    #[error("UnexpectedEndAfterFugitiveKeywordInsideModule")]
    UnexpectedEndAfterFugitiveKeywordInsideModule,
    #[error("UnexpectedEndAfterFugitiveKeywordInsideTrait")]
    UnexpectedEndAfterFugitiveKeywordInsideTrait,
    #[error("UnexpectedEndAfterFugitiveKeywordInsideTypeImplBlock")]
    UnexpectedEndAfterFugitiveKeywordInsideTypeImplBlock,
    #[error("UnexpectedEndAfterFugitiveKeywordInsideTraitForTypeImplBlock")]
    UnexpectedEndAfterFugitiveKeywordInsideTraitForTypeImplBlock,
    #[error("UnexpectedStmtInsideTrait")]
    UnexpectedStmtInsideTrait,
    #[error("UnexpectedStmtInsideModule")]
    UnexpectedStmtInsideModule,
    #[error("UnexpectedMainInsideTrait")]
    UnexpectedMainInsideTrait,
    #[error("UnexpectedUseInsideTrait")]
    UnexpectedUseInsideTrait,
    #[error("UnexpectedModInsideTrait")]
    UnexpectedModInsideTrait,
    #[error("UnexpectedVisualInsideTrait")]
    UnexpectedVisualInsideTrait,
    #[error("UnexpectedImplInsideTrait")]
    UnexpectedImplInsideTrait,
    #[error("UnexpectedTraitInsideTrait")]
    UnexpectedTraitInsideTrait,
    #[error("UnexpectedPattern")]
    UnexpectedPattern,
    #[error("UnexpectedModInsideForm")]
    UnexpectedModInsideForm,
    #[error("UnexpectedVisualInsideForm")]
    UnexpectedVisualInsideForm,
    #[error("UnexpectedImplInsideForm")]
    UnexpectedImplInsideForm,
    #[error("UnexpectedTraitInsideForm")]
    UnexpectedTraitInsideForm,
    #[error("UnexpectedEndKeywordAsFirstNonCommentToken")]
    UnexpectedEndKeywordAsFirstNonCommentToken,
    #[error("UnexpectedTypeDefnInsideTypeImplBlock")]
    UnexpectedTypeDefnInsideTypeImplBlock,
    #[error("VisibilityExprError")]
    VisibilityExprError(#[from] OriginalVisibilityExprError),
    #[error("UnexpectedMemoFieldInsideForm")]
    UnexpectedMemoFieldInsideForm,
    #[error("ExpectedTypeItems")]
    ExpectedTypeItems(TokenGroupIdx),
    #[error("ExpectedTypeVariants")]
    ExpectedTypeVariants(TokenGroupIdx),
    #[error("ExpectedIdentForTypeVariant")]
    ExpectedIdentForTypeVariant(TokenStreamState),
    #[error("ExpectedFormBodyForConfig")]
    ExpectedFormBodyForConfig(TokenGroupIdx),
    #[error("ExpectedFormBodyForMain")]
    ExpectedFormBodyForMain(TokenGroupIdx),
}

impl From<std::convert::Infallible> for AstError {
    fn from(value: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

impl IntoError for OriginalAstError {
    type Error = AstError;
}

impl From<TokenError> for AstError {
    fn from(value: TokenError) -> Self {
        AstError::Derived(value.into())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = AstDb)]
pub enum DerivedAstError {
    #[error("{0}")]
    Token(#[from] TokenError),
    #[error("VisibilityExprError")]
    VisibilityExprError(#[from] DerivedVisibilityExprError),
}

impl From<&AstError> for AstError {
    fn from(value: &AstError) -> Self {
        todo!()
    }
}

pub type AstResult<T> = Result<T, AstError>;

impl From<VisibilityExprError> for AstError {
    fn from(e: VisibilityExprError) -> Self {
        match e {
            VisibilityExprError::Original(e) => AstError::Original(e.into()),
            VisibilityExprError::Derived(e) => AstError::Derived(e.into()),
        }
    }
}
