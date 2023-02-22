use husky_token::{
    IdentifierToken, TokenError, TokenGroupIdx, TokenIdx, TokenIdxRange, TokenParseContext,
};
use parsec::{FromAbsent, HasParseError};
use thiserror::Error;

use crate::{AstDb, AstIdx};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum AstError {
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

impl<'a, Context> FromAbsent<IdentifierToken, Context> for AstError
where
    Context: TokenParseContext<'a>,
{
    fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
        AstError::ExpectIdentifier(state)
    }
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
