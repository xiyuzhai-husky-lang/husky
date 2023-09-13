use super::*;
use husky_opr::{BinaryOpr, Bracket};
use paste::paste;

// punctuation in general

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct PunctuationToken {
    punc: Punctuation,
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for PunctuationToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Punctuation(punc) => Ok(Some(PunctuationToken { punc, token_idx })),
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Ident(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_)
                | TokenData::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

macro_rules! define_specific_punctuation_token {
    ($ty: ident, $punc: ident, $test_name: ident, $s: literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[salsa::debug_with_db(db = TokenDb)]
        pub struct $ty(pub(super) TokenIdx);

        impl $ty {
            pub fn token_idx(self) -> TokenIdx {
                self.0
            }
        }

        impl<'a, Context> parsec::TryParseOptionFromStream<Context> for $ty
        where
            Context: TokenStreamParser<'a>,
        {
            type Error = TokenDataError;

            fn try_parse_option_from_stream_without_guaranteed_rollback(
                ctx: &mut Context,
            ) -> TokenDataResult<Option<Self>> {
                parse_specific_punctuation_from(ctx, Punctuation::$punc, $ty)
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

// specific punctuation

fn parse_specific_punctuation_from<'a, Context, T>(
    ctx: &mut Context,
    target: Punctuation,
    f: impl FnOnce(TokenIdx) -> T,
) -> TokenDataResult<Option<T>>
where
    Context: TokenStreamParser<'a>,
{
    if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
        match token {
            TokenData::Punctuation(punc) if punc == target => Ok(Some(f(token_idx))),
            TokenData::Error(error) => Err(error),
            TokenData::Label(_)
            | TokenData::Punctuation(_)
            | TokenData::Ident(_)
            | TokenData::WordOpr(_)
            | TokenData::Literal(_)
            | TokenData::Keyword(_) => Ok(None),
        }
    } else {
        Ok(None)
    }
}

define_specific_punctuation_token!(ColonToken, COLON, colon_token_works, ":");

define_specific_punctuation_token!(SemiColonToken, SEMICOLON, semicolon_token_works, ";");

define_specific_punctuation_token!(CommaToken, COMMA, comma_token_works, ",");

define_specific_punctuation_token!(EqToken, EQ, eq_token_works, "=");

define_specific_punctuation_token!(LparToken, LPAR, lpar_token_works, "(");

define_specific_punctuation_token!(RparToken, RPAR, rpar_token_works, ")");

define_specific_punctuation_token!(LboxToken, LBOX, lbox_token_works, "[");

define_specific_punctuation_token!(RboxToken, RBOX, rbox_token_works, "]");

define_specific_punctuation_token!(LcurlToken, LCURL, lcurl_token_works, "{");

define_specific_punctuation_token!(RcurlToken, RCURL, rcurl_token_works, "}");

define_specific_punctuation_token!(LaOrLtToken, LA_OR_LT, la_or_lt_token_works, "<");

define_specific_punctuation_token!(
    ColonColonLaToken,
    COLON_COLON_LA,
    colon_colon_la_token_works,
    "::<"
);

define_specific_punctuation_token!(RaOrGtToken, RA_OR_GT, ra_or_gt_token_works, ">");

define_specific_punctuation_token!(
    EmptyHtmlKetToken,
    EMPTY_HTML_KET,
    empty_html_ket_token_works,
    "/>"
);

define_specific_punctuation_token!(VerticalToken, VERTICAL, vertical_token_works, "|");

define_specific_punctuation_token!(AtToken, AT, at_token_works, "@");

define_specific_punctuation_token!(DotDotToken, DOT_DOT, dot_dot_token_works, "..");

define_specific_punctuation_token!(DotDotDotToken, DOT_DOT_DOT, dot_dot_dot_token_works, "...");

define_specific_punctuation_token!(AmbersandToken, AMBERSAND, ambersand_works, "&");

define_specific_punctuation_token!(TildeToken, TILDE, tilde_token_works, "~");

define_specific_punctuation_token!(
    ColonColonToken,
    COLON_COLON,
    scope_resolution_token_works,
    "::"
);

define_specific_punctuation_token!(StarToken, STAR, star_token_works, "*");

define_specific_punctuation_token!(LightArrowToken, LIGHT_ARROW, light_arrow_token_works, "->");

define_specific_punctuation_token!(HeavyArrowToken, HEAVY_ARROW, heavy_arrow_token_works, "->");

define_specific_punctuation_token!(
    DoubleExclamationToken,
    DOUBLE_EXCLAMATION,
    double_exclamation_token_works,
    "!!"
);

define_specific_punctuation_token!(ColonEqToken, COLON_EQ, colon_eq_token_works, ":=");

/// `:` at the end of line
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub enum EolToken {
    Colon(EolColonToken),
    Semicolon(EolSemicolonToken),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct EolColonToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct EolSemicolonToken {
    token_idx: TokenIdx,
}

impl EolToken {
    pub fn token_idx(&self) -> TokenIdx {
        match self {
            EolToken::Colon(token) => token.token_idx,
            EolToken::Semicolon(token) => token.token_idx,
        }
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for EolToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                TokenData::Punctuation(Punctuation::COLON) => match token_stream.peek() {
                    Some(_) => Ok(None),
                    None => Ok(Some(EolToken::Colon(EolColonToken { token_idx }))),
                },
                TokenData::Punctuation(Punctuation::SEMICOLON) => match token_stream.peek() {
                    Some(_) => Ok(None),
                    None => Ok(Some(EolToken::Semicolon(EolSemicolonToken { token_idx }))),
                },
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Punctuation(_)
                | TokenData::Ident(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_)
                | TokenData::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for EolSemicolonToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                TokenData::Punctuation(Punctuation::SEMICOLON) => match token_stream.peek() {
                    Some(_) => Ok(None),
                    None => Ok(Some(EolSemicolonToken { token_idx })),
                },
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Punctuation(_)
                | TokenData::Ident(_)
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
fn eol_colon_token_works() {
    fn t(db: &DB, input: &str) -> TokenDataResult<Option<EolToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ":").unwrap().is_some());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}
