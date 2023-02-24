use husky_token::{
    IdentifierToken, TokenError, TokenGroupIdx, TokenIdx, TokenIdxRange, TokenParseContext,
};
use parsec::OriginalError;
use thiserror::Error;

use crate::{AstDb, AstIdx};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum AstError {
    #[error("{0}")]
    Original(#[from] OriginalAstError),
    #[error("{0}")]
    Derived(#[from] DerivedAstError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum OriginalAstError {
    #[error("excessive indent")]
    ExcessiveIndent,
    #[error("standalone elif")]
    StandaloneElif,
    #[error("standalone else")]
    StandaloneElse,
    #[error("expected entity keyword")]
    ExpectEntityKeyword,
    #[error("expected decorator or entity keyword")]
    ExpectDecoratorOrEntityKeyword,
    #[error("expected identifier")]
    ExpectIdentifier(TokenIdx),
    #[error("unexpected end after `pub`")]
    UnexpectedEndOfTokenGroupAfterPubKeyword(TokenIdx),
    #[error("expected nothing")]
    ExpectNothing,
    #[error("unexpected stmt inside module")]
    UnexpectedStmtInsideModule,
    #[error("unexpected stmt inside impl")]
    UnexpectedStmtInsideImpl,
    #[error("unexpected token for trait item")]
    UnexpectedTokenForTraitItem(TokenIdx),
    #[error("unexpected token for type implementation item")]
    UnexpectedTokenForTypeImplItem(TokenIdx),
    #[error("unexpected token for trait implementation item")]
    UnexpectedTokenForTraitImplItem(TokenIdx),
    #[error("unexpected token for module item")]
    UnexpectedTokenForModuleItem(TokenIdx),
    #[error("invalid ast for definition or use")]
    InvalidAstForDefinitionOrUse,
    #[error("todo")]
    Todo,
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
pub enum DerivedAstError {
    #[error("{0}")]
    Token(#[from] TokenError),
}

impl salsa::DebugWithDb<dyn AstDb + '_> for AstError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn AstDb,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl<Db: AstDb> salsa::DebugWithDb<Db> for AstError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn AstDb, level)
    }
}

impl From<&AstError> for AstError {
    fn from(value: &AstError) -> Self {
        todo!()
    }
}

pub type AstResult<T> = Result<T, AstError>;
