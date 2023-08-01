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
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Punctuation(punc) => Ok(Some(PunctuationToken { punc, token_idx })),
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Ident(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

// specific punctuation

fn parse_specific_punctuation_from<'a, Context, T>(
    ctx: &mut Context,
    target: Punctuation,
    f: impl FnOnce(TokenIdx) -> T,
) -> TokenResult<Option<T>>
where
    Context: TokenStreamParser<'a>,
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
            type Error = TokenError;

            fn try_parse_option_from_stream_without_guaranteed_rollback(
                ctx: &mut Context,
            ) -> TokenResult<Option<Self>> {
                parse_specific_punctuation_from(ctx, Punctuation::$punc, $ty)
            }
        }
        #[test]
        fn $test_name() {
            fn t(db: &DB, input: &str) -> TokenResult<Option<SemiColonToken>> {
                quick_parse(db, input)
            }

            let db = DB::default();
            assert!(t(&db, $s).unwrap().is_some());
        }
    };
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

define_specific_punctuation_token!(TildeToken, TILDE, tilde_token_works, "&");

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
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::COLON) => match token_stream.peek() {
                    Some(_) => Ok(None),
                    None => Ok(Some(EolToken::Colon(EolColonToken { token_idx }))),
                },
                Token::Punctuation(Punctuation::SEMICOLON) => match token_stream.peek() {
                    Some(_) => Ok(None),
                    None => Ok(Some(EolToken::Semicolon(EolSemicolonToken { token_idx }))),
                },
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
}

#[test]
fn eol_colon_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<EolToken>> {
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

/// `::`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct ScopeResolutionToken(TokenIdx);

impl ScopeResolutionToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for ScopeResolutionToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::COLON_COLON) => {
                    Ok(Some(ScopeResolutionToken(token_idx)))
                }
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
}

#[test]
fn scope_resolution_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<ScopeResolutionToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "::").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_some());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `*`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct StarToken(TokenIdx);

impl StarToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for StarToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::STAR) => Ok(Some(StarToken(token_idx))),
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
}

#[test]
fn star_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<StarToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "*").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `->`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct CurryToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for CurryToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::LIGHT_ARROW) => Ok(Some(CurryToken(token_idx))),
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
}

#[test]
fn curry_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<CurryToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "->").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `!!`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct OwnedToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for OwnedToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::DOUBLE_EXCLAMATION) => {
                    Ok(Some(OwnedToken(token_idx)))
                }
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
}

#[test]
fn double_exclamation_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<OwnedToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "!!").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `:=`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct ColonEqToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for ColonEqToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::COLON_EQ, ColonEqToken)
    }
}

#[test]
fn colon_eq_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<ColonEqToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ":=").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `->`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct LightArrowToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for LightArrowToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::LIGHT_ARROW, LightArrowToken)
    }
}

#[test]
fn light_arrow_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<LightArrowToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "->").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `=>`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct HeavyArrowToken(TokenIdx);

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for HeavyArrowToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::HEAVY_ARROW, HeavyArrowToken)
    }
}

#[test]
fn heavy_arrow_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<HeavyArrowToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "=>").unwrap().is_some());
    assert!(t(&db, "->").unwrap().is_none());
    assert!(t(&db, "::@").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}
