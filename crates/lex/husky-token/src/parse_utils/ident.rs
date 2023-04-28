use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct IdentToken {
    ident: Ident,
    token_idx: TokenIdx,
}

impl IdentToken {
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

impl<'a, Context> parsec::ParseFromStream<Context> for IdentToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
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
