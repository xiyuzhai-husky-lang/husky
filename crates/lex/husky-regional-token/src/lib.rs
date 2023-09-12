mod keyword;

pub use self::keyword::*;

use husky_coword::Ident;
use husky_token::TokenGroupStartingTokenIdx;
use husky_token::*;
use husky_token_data::{db::TokenDataDb, *};
use parsec::{HasStreamState, StreamParser, TryParseOptionFromStream};
use std::num::NonZeroU32;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenIdx(NonZeroU32);

impl RegionalTokenIdx {
    pub fn index(self) -> usize {
        self.0.get() as usize - 1
    }

    pub fn token_idx(self, base: TokenRegionBase) -> TokenIdx {
        unsafe { TokenIdx::from_usize_index_ext(self.index() + base.0) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenIdxRange {
    start: (),
    end: (),
}

/// equal to the value of TokenIdx::index on the starting token
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenRegionBase(usize);

impl TokenRegionBase {
    pub(crate) fn new(token_group_base: TokenGroupStartingTokenIdx) -> Self {
        Self(token_group_base.token_idx().index())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTokenGroupIdx {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RegionalTokenStreamState {
    next_token_idx: RegionalTokenIdx,
    drained: bool,
}

impl RegionalTokenStreamState {
    pub fn next_token_idx(&self) -> RegionalTokenIdx {
        self.next_token_idx
    }

    pub fn drained(&self) -> bool {
        self.drained
    }
}

pub struct RegionalTokenStream<'a> {
    tokens: &'a [Token],
}

impl<'a> RegionalTokenStream<'a> {
    pub fn next_indexed(&mut self) -> Option<(RegionalTokenIdx, Token)> {
        todo!()
    }
}

impl<'a> HasStreamState for RegionalTokenStream<'a> {
    type State = RegionalTokenStreamState;

    fn save_state(&self) -> Self::State {
        todo!()
    }

    fn rollback(&mut self, state: Self::State) {
        todo!()
    }
}

macro_rules! define_regional_specific_punctuation_token {
    ($ty: ident, $punc: ident, $test_name: ident, $s: literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[salsa::debug_with_db(db = TokenDataDb)]
        pub struct $ty(pub(crate) RegionalTokenIdx);

        impl $ty {
            pub fn token_idx(self) -> RegionalTokenIdx {
                self.0
            }
        }

        impl<'a, Context> parsec::TryParseOptionFromStream<Context> for $ty
        where
            Context: RegionalTokenStreamParser<'a>,
        {
            type Error = TokenDataError;

            fn try_parse_option_from_stream_without_guaranteed_rollback(
                ctx: &mut Context,
            ) -> TokenDataResult<Option<Self>> {
                parse_regional_specific_punctuation_from(ctx, Punctuation::$punc, $ty)
            }
        }
        #[test]
        fn $test_name() {
            fn t(db: &DB, input: &str) -> TokenDataResult<Option<$ty>> {
                quick_parse(db, input)
            }

            let db = DB::default();
            assert!(t(&db, $s).unwrap().is_some());
        }
    };
}

pub trait RegionalTokenStreamParser<'a>:
    HasStreamState<State = RegionalTokenStreamState>
    + StreamParser
    + core::borrow::BorrowMut<RegionalTokenStream<'a>>
{
    fn token_stream(&self) -> &RegionalTokenStream<'a> {
        self.borrow()
    }

    fn token_stream_mut(&mut self) -> &mut RegionalTokenStream<'a> {
        self.borrow_mut()
    }
}

// impl<'a> TokenParseContext<'a> for TokenIter<'a> {}

impl<'a, T> RegionalTokenStreamParser<'a> for T where
    T: HasStreamState<State = RegionalTokenStreamState>
        + StreamParser
        + core::borrow::BorrowMut<RegionalTokenStream<'a>>
{
}

#[cfg(test)]
fn quick_parse<T, Error>(db: &DB, input: &str) -> Result<Option<T>, Error>
where
    T: for<'a> TryParseOptionFromStream<RegionalTokenStream<'a>, Error = Error>,
{
    use parsec::TryParseOptionFromStream;

    let token_sheet = db.snippet_token_sheet_data(Snippet::new(db, input.to_owned()));
    todo!("get regional token stream")
    // let mut stream = token_sheet
    //     .token_group_token_stream(token_sheet.token_group_iter().next().unwrap().0, None);
    // stream.try_parse_option()
}

// specific punctuation

fn parse_regional_specific_punctuation_from<'a, Context, T>(
    ctx: &mut Context,
    target: Punctuation,
    f: impl FnOnce(RegionalTokenIdx) -> T,
) -> TokenDataResult<Option<T>>
where
    Context: RegionalTokenStreamParser<'a>,
{
    if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
        match token {
            Token::Punctuation(punc) if punc == target => Ok(Some(f(token_idx))),
            Token::Error(error) => Err(error),
            Token::Label(_)
            | Token::Punctuation(_)
            | Token::Ident(_)
            | Token::WordOpr(_)
            | Token::Literal(_)
            | Token::Keyword(_) => Ok(None),
        }
    } else {
        Ok(None)
    }
}

define_regional_specific_punctuation_token!(
    RegionalColonToken,
    COLON,
    regional_colon_token_works,
    ":"
);

define_regional_specific_punctuation_token!(
    SemiColonToken,
    SEMICOLON,
    regional_semicolon_token_works,
    ";"
);

define_regional_specific_punctuation_token!(
    RegionalCommaToken,
    COMMA,
    regional_comma_token_works,
    ","
);

define_regional_specific_punctuation_token!(RegionalEqToken, EQ, regional_eq_token_works, "=");

define_regional_specific_punctuation_token!(
    RegionalLparToken,
    LPAR,
    regional_lpar_token_works,
    "("
);

define_regional_specific_punctuation_token!(
    RegionalRparToken,
    RPAR,
    regional_rpar_token_works,
    ")"
);

define_regional_specific_punctuation_token!(
    RegionalLboxToken,
    LBOX,
    regional_lbox_token_works,
    "["
);

define_regional_specific_punctuation_token!(
    RegionalRboxToken,
    RBOX,
    regional_rbox_token_works,
    "]"
);

define_regional_specific_punctuation_token!(
    RegionalLcurlToken,
    LCURL,
    regional_lcurl_token_works,
    "{"
);

define_regional_specific_punctuation_token!(
    RegionalRcurlToken,
    RCURL,
    regional_rcurl_token_works,
    "}"
);

define_regional_specific_punctuation_token!(
    RegionalLaOrLtToken,
    LA_OR_LT,
    regional_la_or_lt_token_works,
    "<"
);

define_regional_specific_punctuation_token!(
    RegionalColonColonLaToken,
    COLON_COLON_LA,
    colon_colon_la_token_works,
    "::<"
);

define_regional_specific_punctuation_token!(
    RegionalRaOrGtToken,
    RA_OR_GT,
    regional_ra_or_gt_token_works,
    ">"
);

define_regional_specific_punctuation_token!(
    RegionalEmptyHtmlKetToken,
    EMPTY_HTML_KET,
    empty_html_ket_token_works,
    "/>"
);

define_regional_specific_punctuation_token!(
    RegionalVerticalToken,
    VERTICAL,
    regional_vertical_token_works,
    "|"
);

define_regional_specific_punctuation_token!(RegionalAtToken, AT, regional_at_token_works, "@");

define_regional_specific_punctuation_token!(
    RegionalDotDotToken,
    DOT_DOT,
    regional_dot_dot_token_works,
    ".."
);

define_regional_specific_punctuation_token!(
    RegionalDotDotDotToken,
    DOT_DOT_DOT,
    regional_dot_dot_dot_token_works,
    "..."
);

define_regional_specific_punctuation_token!(
    RegionalAmbersandToken,
    AMBERSAND,
    regional_ambersand_works,
    "&"
);

define_regional_specific_punctuation_token!(
    RegionalTildeToken,
    TILDE,
    regional_tilde_token_works,
    "~"
);

define_regional_specific_punctuation_token!(
    RegionalLightArrowToken,
    LIGHT_ARROW,
    regional_light_arrow_token_works,
    "~"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDataDb)]
pub struct RegionalIdentToken {
    pub(crate) ident: Ident,
    pub(crate) regional_token_idx: RegionalTokenIdx,
}

impl RegionalIdentToken {
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

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for RegionalIdentToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                Token::Ident(ident) => Ok(Some(RegionalIdentToken {
                    ident,
                    regional_token_idx,
                })),
                Token::Error(error) => Err(error)?,
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

pub struct RegionalEphemSymbolModifierTokenGroup {}
