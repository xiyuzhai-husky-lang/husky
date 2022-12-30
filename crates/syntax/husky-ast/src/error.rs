use husky_text::TextRange;
use husky_token::{IdentifierToken, TokenError, TokenIdx, TokenParseContext};
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
    #[error("expect entity keyword")]
    ExpectEntityKeyword,
    #[error("expect decorator or entity keyword")]
    ExpectDecoratorOrEntityKeyword,
    #[error("expect identifier")]
    ExpectIdentifier(TokenIdx),
    #[error("expect `(` or decorator or identifier")]
    ExpectParBraOrDecoratorOrIdentifier(Option<TextRange>),
    #[error("expect nothing")]
    ExpectNothing,
    #[error("token error")]
    Token(#[from] TokenError),
}

impl<'a, Context> FromAbsent<IdentifierToken, Context> for AstError
where
    Context: TokenParseContext<'a>,
    <Context as HasParseError>::Error: From<TokenError>,
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
        include_all_fields: bool,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl<Db: AstDb> salsa::DebugWithDb<Db> for AstError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn AstDb, include_all_fields)
    }
}

impl From<&AstError> for AstError {
    fn from(value: &AstError) -> Self {
        todo!()
    }
}

pub type AstResult<T> = Result<T, AstError>;
