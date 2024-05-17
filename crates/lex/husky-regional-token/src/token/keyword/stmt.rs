use super::*;

// let

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct LetRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl LetRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct ReturnRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl ReturnRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct RequireRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl RequireRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct AssertRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl AssertRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct BreakRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl BreakRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct StmtForRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl StmtForRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct ForextRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl ForextRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct WhileRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl WhileRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct DoRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl DoRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct NarrateRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl NarrateRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum BasicStmtKeywordRegionalToken {
    Let(LetRegionalToken),
    Return(ReturnRegionalToken),
    Require(RequireRegionalToken),
    Assert(AssertRegionalToken),
    Break(BreakRegionalToken),
    For(StmtForRegionalToken),
    ForExt(ForextRegionalToken),
    While(WhileRegionalToken),
    Do(DoRegionalToken),
    Narrate(NarrateRegionalToken),
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for WhileRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Stmt(StmtKeyword::While)) => {
                    Ok(Some(WhileRegionalToken { regional_token_idx }))
                }
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

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for BasicStmtKeywordRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Stmt(stmt_keyword)) => match stmt_keyword {
                    StmtKeyword::Let => Ok(Some(LetRegionalToken { regional_token_idx }.into())),
                    StmtKeyword::Return => {
                        Ok(Some(ReturnRegionalToken { regional_token_idx }.into()))
                    }
                    StmtKeyword::Require => {
                        Ok(Some(RequireRegionalToken { regional_token_idx }.into()))
                    }
                    StmtKeyword::Assert => {
                        Ok(Some(AssertRegionalToken { regional_token_idx }.into()))
                    }
                    StmtKeyword::Break => {
                        Ok(Some(BreakRegionalToken { regional_token_idx }.into()))
                    }
                    StmtKeyword::NonImplFor => {
                        Ok(Some(StmtForRegionalToken { regional_token_idx }.into()))
                    }
                    StmtKeyword::Forext => {
                        Ok(Some(ForextRegionalToken { regional_token_idx }.into()))
                    }
                    StmtKeyword::While => {
                        Ok(Some(WhileRegionalToken { regional_token_idx }.into()))
                    }
                    StmtKeyword::Do => Ok(Some(DoRegionalToken { regional_token_idx }.into())),
                    StmtKeyword::If
                    | StmtKeyword::Elif
                    | StmtKeyword::Else
                    | StmtKeyword::Match => Ok(None),
                },
                TokenData::Punctuation(Punctuation::COLON_HYPHEN) => {
                    Ok(Some(NarrateRegionalToken { regional_token_idx }.into()))
                }
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Keyword(_)
                | TokenData::Punctuation(_)
                | TokenData::Ident(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MatchRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl MatchRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for MatchRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Stmt(StmtKeyword::Match)) => {
                    Ok(Some(MatchRegionalToken { regional_token_idx }))
                }
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IfRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl IfRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for IfRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Stmt(StmtKeyword::If)) => {
                    Ok(Some(IfRegionalToken { regional_token_idx }))
                }
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
pub struct ElifRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl ElifRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for ElifRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Stmt(StmtKeyword::Elif)) => {
                    Ok(Some(ElifRegionalToken { regional_token_idx }))
                }
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

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ElseRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl ElseRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for ElseRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Stmt(StmtKeyword::Else)) => {
                    Ok(Some(ElseRegionalToken { regional_token_idx }))
                }
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
pub struct EolWithRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for EolWithRegionalToken
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
                TokenData::Keyword(Keyword::End(EndKeyword::With)) => match token_stream.next() {
                    Some(_) => Ok(None),
                    None => Ok(Some(EolWithRegionalToken { regional_token_idx })),
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
