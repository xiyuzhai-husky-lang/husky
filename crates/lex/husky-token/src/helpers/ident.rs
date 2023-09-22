use husky_token_data::db::HasTokenDataDb;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct IdentToken {
    pub(in crate::helpers) ident: Ident,
    pub(in crate::helpers) token_idx: TokenIdx,
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

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for IdentToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Ident(ident) => Ok(Some(IdentToken { ident, token_idx })),
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Punctuation(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_)
                | TokenData::Keyword(_) => Ok(None),
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
pub struct UnderscoreToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for UnderscoreToken
where
    Context: TokenStreamParser<'a> + HasTokenDataDb<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Ident(ident) => match ident.data(ctx.token_data_db()) {
                    "_" => Ok(Some(Self { token_idx })),
                    _ => Ok(None),
                },
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Punctuation(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_)
                | TokenData::Keyword(_) => Ok(None),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub enum AttrIdentToken {
    Derive(DeriveToken),
}

// "derive"

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeriveToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for DeriveToken
where
    Context: TokenStreamParser<'a> + HasTokenDataDb<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Ident(ident) => match ident.data(ctx.token_data_db()) {
                    "derive" => Ok(Some(Self { token_idx })),
                    _ => Ok(None),
                },
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Punctuation(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_)
                | TokenData::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn derive_token_works() {
    // todo
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PhantomToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for PhantomToken
where
    Context: TokenStreamParser<'a> + HasTokenDataDb<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Ident(ident) => match ident.data(ctx.token_data_db()) {
                    "phantom" => Ok(Some(Self { token_idx })),
                    _ => Ok(None),
                },
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Punctuation(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_)
                | TokenData::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
