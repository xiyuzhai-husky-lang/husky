use super::*;

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

    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for IdentifierToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, Self::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed(IgnoreComment::True) {
            match token.kind {
                TokenKind::Identifier(ident) => Ok(Some(IdentifierToken { ident, token_idx })),
                TokenKind::Comment => unreachable!(),
                TokenKind::Err(ref e) => Err(e.clone()),
                TokenKind::Punctuation(_)
                | TokenKind::WordOpr(_)
                | TokenKind::Literal(_)
                | TokenKind::Attr(_)
                | TokenKind::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
