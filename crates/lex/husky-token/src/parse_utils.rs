use crate::*;
use std::convert::Infallible;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdentifierToken {
    ident: Identifier,
    token_idx: TokenIdx,
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

    pub fn token_idx(&self) -> &TokenIdx {
        &self.token_idx
    }
}

impl<'a> parsec::ParseFrom<TokenIter<'a>> for IdentifierToken {
    type Output = Self;

    type Error = TokenError;

    fn parse_from(token_iter: &mut TokenIter<'a>) -> Result<Option<Self::Output>, Self::Error> {
        while let Some((token_idx, token)) = token_iter.next_indexed() {
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
