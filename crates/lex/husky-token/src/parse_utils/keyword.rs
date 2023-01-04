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

pub enum BasicStmtKeywordToken {
    Let(LetToken),
    Return(ReturnToken),
    Require(RequireToken),
    Break(BreakToken),
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

impl<'a, Context> parsec::ParseFrom<Context> for BasicStmtKeywordToken
where
    Context: TokenParseContext<'a>,
    <Context as HasParseError>::Error: From<TokenError>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed(IgnoreComment::True) {
            match token.kind {
                TokenKind::Keyword(Keyword::Stmt(StmtKeyword::Let)) => {
                    Ok(Some(LetToken { token_idx }.into()))
                }
                TokenKind::Keyword(Keyword::Stmt(StmtKeyword::Return)) => {
                    Ok(Some(ReturnToken { token_idx }.into()))
                }
                TokenKind::Keyword(Keyword::Stmt(StmtKeyword::Require)) => {
                    Ok(Some(RequireToken { token_idx }.into()))
                }
                TokenKind::Keyword(Keyword::Stmt(StmtKeyword::Break)) => {
                    Ok(Some(BreakToken { token_idx }.into()))
                }
                TokenKind::Comment => unreachable!(),
                TokenKind::Err(ref e) => Err(e.clone().into()),
                TokenKind::Keyword(_)
                | TokenKind::Punctuation(_)
                | TokenKind::Identifier(_)
                | TokenKind::WordOpr(_)
                | TokenKind::Literal(_)
                | TokenKind::Attr(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
