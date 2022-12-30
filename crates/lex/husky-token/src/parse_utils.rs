use parsec::HasParseState;

use crate::*;
use std::convert::Infallible;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdentifierToken {
    ident: Identifier,
    token_idx: TokenIdx,
}

pub trait TokenParseContext<'a>: HasParseState {
    fn token_iter(&self) -> &TokenIter<'a>;
    fn token_iter_mut(&mut self) -> &mut TokenIter<'a>;
}

impl<'a> TokenParseContext<'a> for TokenIter<'a> {
    fn token_iter(&self) -> &TokenIter<'a> {
        self
    }

    fn token_iter_mut(&mut self) -> &mut TokenIter<'a> {
        self
    }
}

impl<'a, T> TokenParseContext<'a> for T
where
    T: std::ops::DerefMut<Target = TokenIter<'a>>,
{
    fn token_iter(&self) -> &TokenIter<'a> {
        self.deref()
    }

    fn token_iter_mut(&mut self) -> &mut TokenIter<'a> {
        self.deref_mut()
    }
}

impl<Db> salsa::DebugWithDb<Db> for IdentifierToken
where
    Db: TokenDb + ?Sized,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TokenJar>>::as_jar_db(db);
        f.debug_struct("IdentifierToken")
            .field("ident", &self.ident.debug_with(db, include_all_fields))
            .field("token_idx", &self.token_idx)
            .finish()
    }
}

impl IdentifierToken {
    pub fn new(ident: Identifier, token_idx: TokenIdx) -> Self {
        Self { ident, token_idx }
    }

    pub fn ident(&self) -> Identifier {
        self.ident
    }

    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for IdentifierToken
where
    Context: TokenParseContext<'a>,
{
    type Output = Self;

    type Error = TokenError;

    fn parse_from(ctx: &mut Context) -> Result<Option<Self::Output>, Self::Error> {
        while let Some((token_idx, token)) = ctx.token_iter_mut().next_indexed() {
            match token.kind {
                TokenKind::Identifier(ident) => {
                    return Ok(Some(IdentifierToken { ident, token_idx }))
                }
                TokenKind::Comment => todo!(),
                TokenKind::Err(ref e) => return Err(e.clone()),
                TokenKind::Special(_)
                | TokenKind::WordOpr(_)
                | TokenKind::Literal(_)
                | TokenKind::Attr(_)
                | TokenKind::Keyword(_) => return Ok(None),
            }
        }
        Ok(None)
    }
}
