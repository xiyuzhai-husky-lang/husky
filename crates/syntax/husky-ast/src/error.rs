use husky_token::{
    IdentToken, Punctuation, TokenError, TokenGroupIdx, TokenIdx, TokenIdxRange, TokenParseContext,
};
use parsec::OriginalError;
use thiserror::Error;

use crate::{AstDb, AstIdx};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = AstDb)]
pub enum AstError {
    #[error("{0}")]
    Original(#[from] OriginalAstError),
    #[error("{0}")]
    Derived(#[from] DerivedAstError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = AstDb)]
pub enum OriginalAstError {
    #[error("excessive indent")]
    ExcessiveIndent,
    #[error("standalone elif")]
    StandaloneElif,
    #[error("standalone else")]
    StandaloneElse,
    #[error("expected entity keyword")]
    ExpectedEntityKeyword,
    #[error("expected decorator or entity keyword")]
    ExpectedDecoratorOrEntityKeyword,
    #[error("expected entity keyword group")]
    ExpectedEntityKeywordGroup(TokenIdx),
    #[error("expected identifier")]
    ExpectedIdent(TokenIdx),
    #[error("unexpected end after `pub`")]
    UnexpectedEndOfTokenGroupAfterPubKeyword(TokenIdx),
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
    UnexpectedPunctuationForConnectedModuleItem(TokenIdx, Punctuation),
    #[error("unexpected token for connected module item")]
    UnexpectedTokenForConnectedModuleItem(TokenIdx),
    #[error("unexpected punctuation `{1}` for disconnected module item")]
    UnexpectedPunctuationForDisconnectedModuleItem(TokenIdx, Punctuation),
    #[error("unexpected token for module item")]
    UnexpectedTokenForDisconnectedModuleItem(TokenIdx),
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
}

impl OriginalError for OriginalAstError {
    type Error = AstError;
}

impl From<TokenError> for AstError {
    fn from(value: TokenError) -> Self {
        AstError::Derived(value.into())
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = AstDb)]
pub enum DerivedAstError {
    #[error("{0}")]
    Token(#[from] TokenError),
}

impl From<&AstError> for AstError {
    fn from(value: &AstError) -> Self {
        todo!()
    }
}

pub type AstResult<T> = Result<T, AstError>;
