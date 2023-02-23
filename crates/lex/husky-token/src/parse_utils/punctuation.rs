use super::*;
use husky_opn_syntax::{BinaryOpr, Bracket};

use husky_print_utils::p;
use parsec::{HasParseError, ParseContext, ParseFrom};

// punctuation in general

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PunctuationToken {
    punc: Punctuation,
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::ParseFrom<Context> for PunctuationToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Punctuation(punc) => Ok(Some(PunctuationToken { punc, token_idx })),
                Token::Err(_)
                | Token::AuxiliaryIdentifier(_)
                | Token::Identifier(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Attr(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

// specific punctuation

fn parse_specific_punctuation_from<'a, Context>(
    ctx: &mut Context,
    target: Punctuation,
) -> Option<TokenIdx>
where
    Context: TokenParseContext<'a>,
{
    if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
        match token {
            Token::Punctuation(punc) if punc == target => Some(token_idx),
            Token::Err(_)
            | Token::AuxiliaryIdentifier(_)
            | Token::Punctuation(_)
            | Token::Identifier(_)
            | Token::WordOpr(_)
            | Token::Literal(_)
            | Token::Attr(_)
            | Token::Keyword(_) => None,
        }
    } else {
        None
    }
}

// colon

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColonToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::ParseFrom<Context> for ColonToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        Ok(parse_specific_punctuation_from(ctx, Punctuation::Colon)
            .map(|token_idx| ColonToken { token_idx }))
    }
}

#[test]
fn colon_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<ColonToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ":").unwrap().is_some());
    assert!(t(&db, ",").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// comma

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommaToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::ParseFrom<Context> for CommaToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        Ok(parse_specific_punctuation_from(ctx, Punctuation::Comma)
            .map(|token_idx| CommaToken { token_idx }))
    }
}

#[test]
fn comma_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<CommaToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ",").unwrap().is_some());
    assert!(t(&db, ")").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// assign

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssignToken {
    token_idx: TokenIdx,
}

impl AssignToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for AssignToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        Ok(
            parse_specific_punctuation_from(ctx, BinaryOpr::Assign(None).into())
                .map(|token_idx| AssignToken { token_idx }),
        )
    }
}

#[test]
fn assign_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<AssignToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "=").unwrap().is_some());
    assert!(t(&db, ")").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// left parenthesis

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftParenthesisToken {
    token_idx: TokenIdx,
}

impl LeftParenthesisToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for LeftParenthesisToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        Ok(
            parse_specific_punctuation_from(ctx, Punctuation::Bra(Bracket::Par))
                .map(|token_idx| LeftParenthesisToken { token_idx }),
        )
    }
}

#[test]
fn left_parenthesis_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<LeftParenthesisToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "(").unwrap().is_some());
    assert!(t(&db, ")").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// right parenthesis

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RightParenthesisToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::ParseFrom<Context> for RightParenthesisToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        Ok(
            parse_specific_punctuation_from(ctx, Punctuation::Ket(Bracket::Par))
                .map(|token_idx| RightParenthesisToken { token_idx }),
        )
    }
}

#[test]
fn right_parenthesis_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<RightParenthesisToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ")").unwrap().is_some());
    assert!(t(&db, "(").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// left box bracket

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftBoxBracketToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::ParseFrom<Context> for LeftBoxBracketToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        Ok(
            parse_specific_punctuation_from(ctx, Punctuation::Bra(Bracket::Box))
                .map(|token_idx| LeftBoxBracketToken { token_idx }),
        )
    }
}

#[test]
fn left_box_bracket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<LeftBoxBracketToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "[").unwrap().is_some());
    assert!(t(&db, "]").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// right box bracket

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RightBoxBracketToken {
    token_idx: TokenIdx,
}

impl RightBoxBracketToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for RightBoxBracketToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        Ok(
            parse_specific_punctuation_from(ctx, Punctuation::Ket(Bracket::Box))
                .map(|token_idx| RightBoxBracketToken { token_idx }),
        )
    }
}

#[test]
fn right_box_bracket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<RightBoxBracketToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "]").unwrap().is_some());
    assert!(t(&db, "[").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// left curly brace

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftCurlyBraceToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::ParseFrom<Context> for LeftCurlyBraceToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        Ok(
            parse_specific_punctuation_from(ctx, Punctuation::Bra(Bracket::Curl))
                .map(|token_idx| LeftCurlyBraceToken { token_idx }),
        )
    }
}

#[test]
fn left_curly_brace_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<LeftCurlyBraceToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "{").unwrap().is_some());
    assert!(t(&db, "}").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// right curly brace

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RightCurlyBraceToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::ParseFrom<Context> for RightCurlyBraceToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        Ok(
            parse_specific_punctuation_from(ctx, Punctuation::Ket(Bracket::Curl))
                .map(|token_idx| RightCurlyBraceToken { token_idx }),
        )
    }
}

#[test]
fn right_curly_brace_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<RightCurlyBraceToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "}").unwrap().is_some());
    assert!(t(&db, "{").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// left angle bracket

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftAngleBracketOrLessThanToken {
    token_idx: TokenIdx,
}

impl LeftAngleBracketOrLessThanToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for LeftAngleBracketOrLessThanToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        Ok(parse_specific_punctuation_from(ctx, Punctuation::LaOrLt)
            .map(|token_idx| LeftAngleBracketOrLessThanToken { token_idx }))
    }
}

#[test]
fn left_angle_or_less_bracket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<LeftAngleBracketOrLessThanToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "<").unwrap().is_some());
    assert!(t(&db, "::<").unwrap().is_none());
    assert!(t(&db, ">").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// colon colon left angle bracket

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColonColonLeftAngleBracketToken {
    token_idx: TokenIdx,
}

impl ColonColonLeftAngleBracketToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for ColonColonLeftAngleBracketToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        Ok(
            parse_specific_punctuation_from(ctx, Punctuation::ColonColonLAngle)
                .map(|token_idx| ColonColonLeftAngleBracketToken { token_idx }),
        )
    }
}

#[test]
fn colon_colon_left_angle_bracket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<ColonColonLeftAngleBracketToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "::<").unwrap().is_some());
    assert!(t(&db, "<").unwrap().is_none());
    assert!(t(&db, ">").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// right curly brace

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RightAngleBracketToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::ParseFrom<Context> for RightAngleBracketToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        Ok(parse_specific_punctuation_from(ctx, Punctuation::RaOrGt)
            .map(|token_idx| RightAngleBracketToken { token_idx }))
    }
}

#[test]
fn right_angle_bracket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<RightAngleBracketToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, ">").unwrap().is_some());
    assert!(t(&db, "<").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// vertical

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerticalToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::ParseFrom<Context> for VerticalToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        Ok(parse_specific_punctuation_from(ctx, Punctuation::Vertical)
            .map(|token_idx| VerticalToken { token_idx }))
    }
}

#[test]
fn vertical_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<VerticalToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "|").unwrap().is_some());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// at

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::ParseFrom<Context> for AtToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        Ok(parse_specific_punctuation_from(ctx, Punctuation::At)
            .map(|token_idx| AtToken { token_idx }))
    }
}

#[test]
fn at_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<AtToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "@").unwrap().is_some());
    assert!(t(&db, "|").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// dotdot

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DotDotToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::ParseFrom<Context> for DotDotToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        Ok(parse_specific_punctuation_from(ctx, Punctuation::DotDot)
            .map(|token_idx| DotDotToken { token_idx }))
    }
}

#[test]
fn dotdot_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<DotDotToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "..").unwrap().is_some());
    assert!(t(&db, "@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

/// `:` at the end of line
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EolColonToken {
    token_idx: TokenIdx,
}

impl EolColonToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for EolColonToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::Colon) => match token_stream.peek() {
                    Some(_) => Ok(None),
                    None => Ok(Some(EolColonToken { token_idx })),
                },
                Token::Err(_)
                | Token::AuxiliaryIdentifier(_)
                | Token::Punctuation(_)
                | Token::Identifier(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Attr(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn eol_colon_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<EolColonToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ":").unwrap().is_some());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

/// `::`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScopeResolutionToken {
    token_idx: TokenIdx,
}

impl ScopeResolutionToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for ScopeResolutionToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::ColonColon) => {
                    Ok(Some(ScopeResolutionToken { token_idx }))
                }
                Token::Err(_)
                | Token::AuxiliaryIdentifier(_)
                | Token::Punctuation(_)
                | Token::Identifier(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Attr(_)
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
}

/// `*`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StarToken {
    token_idx: TokenIdx,
}

impl StarToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for StarToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::Star) => Ok(Some(StarToken { token_idx })),
                Token::Err(_)
                | Token::AuxiliaryIdentifier(_)
                | Token::Punctuation(_)
                | Token::Identifier(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Attr(_)
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
}

/// `->`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurryToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::ParseFrom<Context> for CurryToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::Binary(BinaryOpr::Curry)) => {
                    Ok(Some(CurryToken { token_idx }))
                }
                Token::Err(_)
                | Token::AuxiliaryIdentifier(_)
                | Token::Punctuation(_)
                | Token::Identifier(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Attr(_)
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
}
