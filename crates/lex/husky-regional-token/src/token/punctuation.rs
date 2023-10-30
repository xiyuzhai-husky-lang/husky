use crate::*;

macro_rules! define_specific_punctuation_regional_token {
    ($ty: ident, $punc: ident, $test_name: ident, $s: literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[salsa::debug_with_db(db = TokenDataDb)]
        pub struct $ty(pub(crate) RegionalTokenIdx);

        impl $ty {
            pub fn regional_token_idx(self) -> RegionalTokenIdx {
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

fn parse_regional_specific_punctuation_from<'a, Context, T>(
    ctx: &mut Context,
    target: Punctuation,
    f: impl FnOnce(RegionalTokenIdx) -> T,
) -> TokenDataResult<Option<T>>
where
    Context: RegionalTokenStreamParser<'a>,
{
    if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
        match token {
            TokenData::Punctuation(punc) if punc == target => Ok(Some(f(regional_token_idx))),
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

define_specific_punctuation_regional_token!(
    ColonRegionalToken,
    COLON,
    colon_regional_token_works,
    ":"
);

define_specific_punctuation_regional_token!(
    SemiColonRegionalToken,
    SEMICOLON,
    semicolon_regional_token_works,
    ";"
);

define_specific_punctuation_regional_token!(
    CommaRegionalToken,
    COMMA,
    comma_regional_token_works,
    ","
);

define_specific_punctuation_regional_token!(EqRegionalToken, EQ, regional_eq_token_works, "=");

define_specific_punctuation_regional_token!(
    LparRegionalToken,
    LPAR,
    lpar_regional_token_works,
    "("
);

define_specific_punctuation_regional_token!(
    RparRegionalToken,
    RPAR,
    rpar_regional_token_works,
    ")"
);

define_specific_punctuation_regional_token!(
    LboxRegionalToken,
    LBOX,
    regional_lbox_token_works,
    "["
);

define_specific_punctuation_regional_token!(
    RboxRegionalToken,
    RBOX,
    regional_rbox_token_works,
    "]"
);

define_specific_punctuation_regional_token!(
    LcurlRegionalToken,
    LCURL,
    regional_lcurl_token_works,
    "{"
);

define_specific_punctuation_regional_token!(
    RcurlRegionalToken,
    RCURL,
    regional_rcurl_token_works,
    "}"
);

define_specific_punctuation_regional_token!(
    LaOrLtRegionalToken,
    LA_OR_LT,
    regional_la_or_lt_token_works,
    "<"
);

define_specific_punctuation_regional_token!(
    ColonColonLaRegionalToken,
    COLON_COLON_LA,
    colon_colon_la_token_works,
    "::<"
);

define_specific_punctuation_regional_token!(
    RaOrGtRegionalToken,
    RA_OR_GT,
    regional_ra_or_gt_token_works,
    ">"
);

define_specific_punctuation_regional_token!(
    EmptyHtmlKetRegionalToken,
    EMPTY_HTML_KET,
    empty_html_ket_token_works,
    "/>"
);

define_specific_punctuation_regional_token!(
    VerticalRegionalToken,
    VERTICAL,
    vertical_regional_token_works,
    "|"
);

define_specific_punctuation_regional_token!(
    PoundRegionalToken,
    POUND,
    pound_regional_token_works,
    "#"
);

define_specific_punctuation_regional_token!(
    DotDotRegionalToken,
    DOT_DOT,
    dot_dot_regional_token_works,
    ".."
);

define_specific_punctuation_regional_token!(AtRegionalToken, AT, at_regional_token_works, "@");

define_specific_punctuation_regional_token!(
    DotDotDotRegionalToken,
    DOT_DOT_DOT,
    dot_dot_dot_regional_token_works,
    "..."
);

define_specific_punctuation_regional_token!(
    AmbersandRegionalToken,
    AMBERSAND,
    regional_ambersand_works,
    "&"
);

define_specific_punctuation_regional_token!(
    TildeRegionalToken,
    TILDE,
    regional_tilde_token_works,
    "~"
);

define_specific_punctuation_regional_token!(
    ColonColonRegionalToken,
    COLON_COLON,
    colon_colon_regional_token_works,
    "::"
);

define_specific_punctuation_regional_token!(
    LightArrowRegionalToken,
    LIGHT_ARROW,
    light_arrow_regional_token_works,
    "->"
);

define_specific_punctuation_regional_token!(
    HeavyArrowRegionalToken,
    HEAVY_ARROW,
    heavy_arrow_regional_token_works,
    "=>"
);

define_specific_punctuation_regional_token!(
    DoubleExclamationRegionalToken,
    DOUBLE_EXCLAMATION,
    double_exclamation_regional_token_works,
    "!!"
);

define_specific_punctuation_regional_token!(
    ColonEqRegionalToken,
    COLON_EQ,
    colon_eq_regional_token_works,
    ":="
);

/// `:` at the end of line
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub enum EolRegionalToken {
    Colon(EolColonRegionalToken),
    Semicolon(EolSemicolonRegionalToken),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct EolColonRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl EolColonRegionalToken {
    pub fn regional_token_idx(self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for EolColonRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((regional_token_idx, token)) = token_stream.next_indexed() {
            match token {
                TokenData::Punctuation(Punctuation::COLON) => match token_stream.peek() {
                    Some(_) => Ok(None),
                    None => Ok(Some(EolColonRegionalToken { regional_token_idx })),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb)]
pub struct EolSemicolonRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl EolRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        match self {
            EolRegionalToken::Colon(token) => token.regional_token_idx,
            EolRegionalToken::Semicolon(token) => token.regional_token_idx,
        }
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for EolRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((regional_token_idx, token)) = token_stream.next_indexed() {
            match token {
                TokenData::Punctuation(Punctuation::COLON) => match token_stream.peek() {
                    Some(_) => Ok(None),
                    None => Ok(Some(EolRegionalToken::Colon(EolColonRegionalToken {
                        regional_token_idx,
                    }))),
                },
                TokenData::Punctuation(Punctuation::SEMICOLON) => match token_stream.peek() {
                    Some(_) => Ok(None),
                    None => Ok(Some(EolRegionalToken::Semicolon(
                        EolSemicolonRegionalToken { regional_token_idx },
                    ))),
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

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for EolSemicolonRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((regional_token_idx, token)) = token_stream.next_indexed() {
            match token {
                TokenData::Punctuation(Punctuation::SEMICOLON) => match token_stream.peek() {
                    Some(_) => Ok(None),
                    None => Ok(Some(EolSemicolonRegionalToken { regional_token_idx })),
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
    fn t(db: &DB, input: &str) -> TokenDataResult<Option<EolRegionalToken>> {
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
