use crate::*;
use husky_scope_expr::{
    DerivedVisibilityExprError, OriginalVisibilityExprError, VisibilityExprError,
};
use husky_token::{TokenIdx, TokenStreamState};
use husky_vfs::error::VfsError;
use original_error::OriginalError;
use thiserror::Error;

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AstError {
    #[error("{0}")]
    Original(#[from] OriginalAstError),
    #[error("{0}")]
    Derived(#[from] DerivedAstError),
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
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
    #[error("UnexpectedEndAfterFormKeywordInsideModule")]
    UnexpectedEndAfterFormKeywordInsideModule,
    #[error("UnexpectedEndAfterFormKeywordInsideTrait")]
    UnexpectedEndAfterFormKeywordInsideTrait,
    #[error("UnexpectedEndAfterFormKeywordInsideTypeImplBlock")]
    UnexpectedEndAfterFormKeywordInsideTypeImplBlock,
    #[error("UnexpectedEndAfterFormKeywordInsideTraitForTypeImplBlock")]
    UnexpectedEndAfterFormKeywordInsideTraitForTypeImplBlock,
    #[error("UnexpectedStmtInsideTrait")]
    UnexpectedStmtInsideTrait,
    #[error("UnexpectedStmtInsideModule")]
    UnexpectedStmtUnderModule,
    #[error("UnexpectedMainInsideTrait")]
    UnexpectedMainInsideTrait,
    #[error("UnexpectedUseInsideTrait")]
    UnexpectedUseInsideTrait,
    #[error("unexpected submodule inside module item")]
    UnexpectedModUnderForm,
    #[error("unexpected implemention block inside module item")]
    UnexpectedImplBlockInsideModuleItem,
    #[error("UnexpectedTraitInsideTrait")]
    UnexpectedTraitInsideTrait,
    #[error("UnexpectedPattern")]
    UnexpectedPattern,
    #[error("unexpected static function outside implementation block")]
    UnexpectedStaticFnOutsideImplBlock,
    #[error("UnexpectedTraitInsideForm")]
    UnexpectedTraitInsideForm,
    #[error("UnexpectedEndKeywordAsFirstNonCommentToken")]
    UnexpectedEndKeywordAsFirstNonCommentToken,
    #[error("unexpected connection keyword as first non-comment token")]
    UnexpectedConnectionKeywordAsFirstNonCommentToken,
    #[error("unexpected major type inside implementation block")]
    UnexpectedMajorTypeInsideImplBlock,
    #[error("unexpected trait inside implementation block")]
    UnexpectedTraitInsideImplBlock,
    #[error("unexpected memomoized field outside implementation block")]
    UnexpectedMemoFieldOutsideImplBlock,
    #[error("ExpectedTypeItems")]
    ExpectedTypeItems(TokenVerseIdx),
    #[error("ExpectedTypeVariants")]
    ExpectedTypeVariants(TokenVerseIdx),
    #[error("ExpectedIdentForTypeVariant")]
    ExpectedIdentForTypeVariant(TokenStreamState),
    #[error("ExpectedFormBodyForConfig")]
    ExpectedFormBodyForConfig(TokenVerseIdx),
    #[error("ExpectedFormBodyForMain")]
    ExpectedFormBodyForMain(TokenVerseIdx),
    #[error("SubmoduleFileNotFound")]
    SubmoduleFileNotFound {
        ident_token: IdentToken,
        error: Option<VfsError>,
    },
    #[error("VisibilityExprError")]
    VisibilityExprError(#[from] OriginalVisibilityExprError),
    #[error("ExpectedLboxOrIdentAfterPoundForAttrOrSorce")]
    ExpectedLboxOrIdentAfterPoundForAttrOrSorce,
    #[error("UnexpectedMemoUnderModule")]
    UnexpectedMemoUnderModule,
    #[error("UnexpectedMemoUnderForm")]
    UnexpectedMemoUnderForm,
    #[error("unexpected const")]
    UnexpectedConst,
}

impl From<std::convert::Infallible> for AstError {
    fn from(_value: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

impl OriginalError for OriginalAstError {
    type Error = AstError;
}

impl From<TokenDataError> for AstError {
    fn from(value: TokenDataError) -> Self {
        AstError::Derived(value.into())
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedAstError {
    #[error("{0}")]
    TokenData(#[from] TokenDataError),
    #[error("VisibilityExprError")]
    VisibilityExprError(#[from] DerivedVisibilityExprError),
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
