use crate::*;

macro_rules! define_specific_punctuation_regional_token {
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

define_specific_punctuation_regional_token!(RegionalEqToken, EQ, regional_eq_token_works, "=");

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
    regional_vertical_token_works,
    "|"
);

define_specific_punctuation_regional_token!(AtRegionalToken, AT, regional_at_token_works, "@");

define_specific_punctuation_regional_token!(
    DotDotRegionalToken,
    DOT_DOT,
    dot_dot_regional_token_works,
    ".."
);

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
    "~"
);

define_specific_punctuation_regional_token!(
    HeavyArrowRegionalToken,
    HEAVY_ARROW,
    heavy_arrow_regional_token_works,
    "->"
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
