use husky_coword::Label;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct LifetimeLabelRegionalToken {
    label: Label,
    token_idx: RegionalTokenIdx,
}

impl LifetimeLabelRegionalToken {
    pub fn label(&self) -> Label {
        self.label
    }

    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for LifetimeLabelRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Label(label) if label.is_valid_lifetime_label() => {
                    Ok(Some(LifetimeLabelRegionalToken { label, token_idx }))
                }
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Ident(_)
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
fn lifetime_label_token_works() {
    // todo
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct PlaceLabelRegionalToken {
    label: Label,
    regional_token_idx: RegionalTokenIdx,
}

impl PlaceLabelRegionalToken {
    pub fn label(&self) -> Label {
        self.label
    }

    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for PlaceLabelRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Label(label) if label.is_valid_place_label() => {
                    Ok(Some(PlaceLabelRegionalToken {
                        label,
                        regional_token_idx,
                    }))
                }
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Ident(_)
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
fn aux_ident_token_works() {
    // todo
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct BlockLabelRegionalToken {
    label: Label,
    token_idx: RegionalTokenIdx,
}

impl BlockLabelRegionalToken {
    pub fn ident(&self) -> Label {
        self.label
    }

    pub fn token_idx(&self) -> RegionalTokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for BlockLabelRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Label(label) => Ok(Some(BlockLabelRegionalToken { label, token_idx })),
                TokenData::Error(error) => Err(error),
                TokenData::Ident(_)
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
fn lifetime_ident_token_works() {
    // todo
}
