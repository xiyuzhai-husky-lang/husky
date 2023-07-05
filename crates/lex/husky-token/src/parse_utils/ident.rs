use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct IdentToken {
    pub(super) ident: Ident,
    pub(super) token_idx: TokenIdx,
}

impl IdentToken {
    pub fn new(ident: Ident, token_idx: TokenIdx) -> Self {
        Self { ident, token_idx }
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ident_ref(&self) -> &Ident {
        &self.ident
    }

    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for IdentToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_stream_optional_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                Token::Ident(ident) => Ok(Some(IdentToken { ident, token_idx })),
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Punctuation(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn ident_token_works() {
    // todo
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub enum DecrIdentToken {
    Derive(DeriveToken),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeriveToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnderscoreToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for UnderscoreToken
where
    Context: TokenParseContext<'a> + HasTokenDb,
{
    type Error = TokenError;

    fn try_parse_stream_optional_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                Token::Ident(ident) => match ident.data(ctx.token_db()) {
                    "_" => Ok(Some(Self { token_idx })),
                    _ => Ok(None),
                },
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Punctuation(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn underscore_token_works() {
    // todo
}
