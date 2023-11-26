use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDataDb, jar = TokenDataJar)]
pub struct IdentRegionalToken {
    pub(crate) ident: Ident,
    pub(crate) regional_token_idx: RegionalTokenIdx,
}

impl IdentRegionalToken {
    pub fn new(ident: Ident, regional_token_idx: RegionalTokenIdx) -> Self {
        Self {
            ident,
            regional_token_idx,
        }
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ident_ref(&self) -> &Ident {
        &self.ident
    }

    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for IdentRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Ident(ident) => Ok(Some(IdentRegionalToken {
                    ident,
                    regional_token_idx,
                })),
                TokenData::Error(error) => Err(error)?,
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
pub struct UnderscoreRegionalToken {
    token_idx: RegionalTokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for UnderscoreRegionalToken
where
    Context: RegionalTokenStreamParser<'a> + ::salsa::db::HasDb<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Ident(ident) => match ident.data(ctx.db()) {
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
#[salsa::debug_with_db(db = TokenDb, jar = TokenJar)]
pub enum AttrIdentRegionalToken {
    Derive(DeriveRegionalToken),
}

// "derive"

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeriveRegionalToken {
    token_idx: RegionalTokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for DeriveRegionalToken
where
    Context: RegionalTokenStreamParser<'a> + ::salsa::db::HasDb<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Ident(ident) => match ident.data(ctx.db()) {
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
fn derive_regional_token_works() {
    // todo
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PhantomRegionalToken {
    token_idx: RegionalTokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for PhantomRegionalToken
where
    Context: RegionalTokenStreamParser<'a> + ::salsa::db::HasDb<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Ident(ident) => match ident.data(ctx.db()) {
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
