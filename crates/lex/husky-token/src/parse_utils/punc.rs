use super::*;
use husky_opn_syntax::{BinaryOpr, Bracket};
use husky_print_utils::p;
use parsec::{ParseContext, ParseFrom};

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
    type Error = TokenError;

    fn parse_from(ctx: &mut Context) -> Result<Option<Self>, Self::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed(IgnoreComment::True) {
            match token.kind {
                TokenKind::Punctuation(punc) => Ok(Some(PunctuationToken { punc, token_idx })),
                TokenKind::Comment => unreachable!(),
                TokenKind::Err(ref e) => Err(e.clone()),
                TokenKind::Identifier(_)
                | TokenKind::WordOpr(_)
                | TokenKind::Literal(_)
                | TokenKind::Attr(_)
                | TokenKind::Keyword(_) => Ok(None),
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
) -> Result<Option<TokenIdx>, TokenError>
where
    Context: TokenParseContext<'a>,
{
    if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed(IgnoreComment::True) {
        match token.kind {
            TokenKind::Punctuation(punc) if punc == target => Ok(Some(token_idx)),
            TokenKind::Comment => unreachable!(),
            TokenKind::Err(ref e) => Err(e.clone()),
            TokenKind::Punctuation(_)
            | TokenKind::Identifier(_)
            | TokenKind::WordOpr(_)
            | TokenKind::Literal(_)
            | TokenKind::Attr(_)
            | TokenKind::Keyword(_) => Ok(None),
        }
    } else {
        Ok(None)
    }
}

#[cfg(test)]
fn quick_parse<T>(db: &DB, input: &str) -> TokenResult<Option<T>>
where
    T: for<'a> ParseFrom<TokenStream<'a>, Error = TokenError>,
{
    let tokens = db.tokenize(input);
    let token_sheet = TokenSheet::new(tokens);
    let mut stream = token_sheet
        .token_group_token_stream(token_sheet.token_group_iter().next().unwrap().0, None);
    stream.parse()
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
    type Error = TokenError;

    fn parse_from(ctx: &mut Context) -> Result<Option<Self>, Self::Error> {
        Ok(parse_specific_punctuation_from(ctx, Punctuation::Comma)?
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

impl<'a, Context> parsec::ParseFrom<Context> for AssignToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from(ctx: &mut Context) -> Result<Option<Self>, Self::Error> {
        Ok(
            parse_specific_punctuation_from(ctx, BinaryOpr::Assign(None).into())?
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

impl<'a, Context> parsec::ParseFrom<Context> for LeftParenthesisToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from(ctx: &mut Context) -> Result<Option<Self>, Self::Error> {
        Ok(
            parse_specific_punctuation_from(ctx, Punctuation::Bra(Bracket::Par))?
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
    type Error = TokenError;

    fn parse_from(ctx: &mut Context) -> Result<Option<Self>, Self::Error> {
        Ok(
            parse_specific_punctuation_from(ctx, Punctuation::Ket(Bracket::Par))?
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

// left curly brace

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftCurlyBraceToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::ParseFrom<Context> for LeftCurlyBraceToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from(ctx: &mut Context) -> Result<Option<Self>, Self::Error> {
        Ok(
            parse_specific_punctuation_from(ctx, Punctuation::Bra(Bracket::Curl))?
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
    type Error = TokenError;

    fn parse_from(ctx: &mut Context) -> Result<Option<Self>, Self::Error> {
        Ok(
            parse_specific_punctuation_from(ctx, Punctuation::Ket(Bracket::Curl))?
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

// vertical

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerticalToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::ParseFrom<Context> for VerticalToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from(ctx: &mut Context) -> Result<Option<Self>, Self::Error> {
        Ok(parse_specific_punctuation_from(ctx, Punctuation::Vertical)?
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
