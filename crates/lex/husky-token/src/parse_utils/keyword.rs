use super::*;
use parsec::HasParseError;

// let

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LetToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReturnToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequireToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BreakToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForextToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhileToken {
    token_idx: TokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoToken {
    token_idx: TokenIdx,
}

pub enum BasicStmtKeywordToken {
    Let(LetToken),
    Return(ReturnToken),
    Require(RequireToken),
    Break(BreakToken),
    For(ForToken),
    ForExt(ForextToken),
    While(WhileToken),
    Do(DoToken),
}

impl From<DoToken> for BasicStmtKeywordToken {
    fn from(v: DoToken) -> Self {
        Self::Do(v)
    }
}

impl From<WhileToken> for BasicStmtKeywordToken {
    fn from(v: WhileToken) -> Self {
        Self::While(v)
    }
}

impl From<ForextToken> for BasicStmtKeywordToken {
    fn from(v: ForextToken) -> Self {
        Self::ForExt(v)
    }
}

impl From<ForToken> for BasicStmtKeywordToken {
    fn from(v: ForToken) -> Self {
        Self::For(v)
    }
}

impl From<BreakToken> for BasicStmtKeywordToken {
    fn from(v: BreakToken) -> Self {
        Self::Break(v)
    }
}

impl From<RequireToken> for BasicStmtKeywordToken {
    fn from(v: RequireToken) -> Self {
        Self::Require(v)
    }
}

impl From<ReturnToken> for BasicStmtKeywordToken {
    fn from(v: ReturnToken) -> Self {
        Self::Return(v)
    }
}

impl From<LetToken> for BasicStmtKeywordToken {
    fn from(v: LetToken) -> Self {
        Self::Let(v)
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for WhileToken
where
    Context: TokenParseContext<'a>,
    <Context as HasParseError>::Error: From<TokenError>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::While)) => {
                    Ok(Some(WhileToken { token_idx }))
                }
                Token::Err(ref e) => Err(e.clone().into()),
                Token::Punctuation(_)
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

impl<'a, Context> parsec::ParseFrom<Context> for BasicStmtKeywordToken
where
    Context: TokenParseContext<'a>,
    <Context as HasParseError>::Error: From<TokenError>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::Let)) => {
                    Ok(Some(LetToken { token_idx }.into()))
                }
                Token::Keyword(Keyword::Stmt(StmtKeyword::Return)) => {
                    Ok(Some(ReturnToken { token_idx }.into()))
                }
                Token::Keyword(Keyword::Stmt(StmtKeyword::Require)) => {
                    Ok(Some(RequireToken { token_idx }.into()))
                }
                Token::Keyword(Keyword::Stmt(StmtKeyword::Break)) => {
                    Ok(Some(BreakToken { token_idx }.into()))
                }
                Token::Keyword(Keyword::Stmt(StmtKeyword::For)) => {
                    Ok(Some(ForToken { token_idx }.into()))
                }
                Token::Keyword(Keyword::Stmt(StmtKeyword::ForExt)) => {
                    Ok(Some(ForextToken { token_idx }.into()))
                }
                Token::Keyword(Keyword::Stmt(StmtKeyword::While)) => {
                    Ok(Some(WhileToken { token_idx }.into()))
                }
                Token::Keyword(Keyword::Stmt(StmtKeyword::Do)) => {
                    Ok(Some(DoToken { token_idx }.into()))
                }
                Token::Err(ref e) => Err(e.clone().into()),
                Token::Keyword(_)
                | Token::Punctuation(_)
                | Token::Identifier(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Attr(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

pub struct MutToken {
    token_idx: TokenIdx,
}

impl MutToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for MutToken
where
    Context: TokenParseContext<'a>,
    <Context as HasParseError>::Error: From<TokenError>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Liason(LiasonKeyword::Mut)) => {
                    Ok(Some(MutToken { token_idx }))
                }
                Token::Err(ref e) => Err(e.clone().into()),
                Token::Punctuation(_)
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

#[derive(Debug, PartialEq, Eq)]
pub struct IfToken {
    token_idx: TokenIdx,
}

impl IfToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for IfToken
where
    Context: TokenParseContext<'a>,
    <Context as HasParseError>::Error: From<TokenError>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::If)) => Ok(Some(IfToken { token_idx })),
                Token::Err(ref e) => Err(e.clone().into()),
                Token::Punctuation(_)
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

#[derive(Debug, PartialEq, Eq)]
pub struct ElifToken {
    token_idx: TokenIdx,
}

impl ElifToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for ElifToken
where
    Context: TokenParseContext<'a>,
    <Context as HasParseError>::Error: From<TokenError>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::Elif)) => {
                    Ok(Some(ElifToken { token_idx }))
                }
                Token::Err(ref e) => Err(e.clone().into()),
                Token::Punctuation(_)
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

#[derive(Debug, PartialEq, Eq)]
pub struct ElseToken {
    token_idx: TokenIdx,
}

impl ElseToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for ElseToken
where
    Context: TokenParseContext<'a>,
    <Context as HasParseError>::Error: From<TokenError>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::Else)) => {
                    Ok(Some(ElseToken { token_idx }))
                }
                Token::Err(ref e) => Err(e.clone().into()),
                Token::Punctuation(_)
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ImplToken {
    token_idx: TokenIdx,
}

impl ImplToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for ImplToken
where
    Context: TokenParseContext<'a>,
    <Context as HasParseError>::Error: From<TokenError>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Impl) => Ok(Some(ImplToken { token_idx })),
                Token::Err(ref e) => Err(e.clone().into()),
                Token::Punctuation(_)
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
