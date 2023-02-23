use super::*;
use parsec::HasParseError;

// let

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LetToken {
    token_idx: TokenIdx,
}

impl LetToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReturnToken {
    token_idx: TokenIdx,
}

impl ReturnToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequireToken {
    token_idx: TokenIdx,
}

impl RequireToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssertToken {
    token_idx: TokenIdx,
}

impl AssertToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BreakToken {
    token_idx: TokenIdx,
}

impl BreakToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForToken {
    token_idx: TokenIdx,
}

impl ForToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForextToken {
    token_idx: TokenIdx,
}

impl ForextToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhileToken {
    token_idx: TokenIdx,
}

impl WhileToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoToken {
    token_idx: TokenIdx,
}

impl DoToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

pub enum BasicStmtKeywordToken {
    Let(LetToken),
    Return(ReturnToken),
    Require(RequireToken),
    Assert(AssertToken),
    Break(BreakToken),
    For(ForToken),
    ForExt(ForextToken),
    While(WhileToken),
    Do(DoToken),
}

impl From<AssertToken> for BasicStmtKeywordToken {
    fn from(v: AssertToken) -> Self {
        Self::Assert(v)
    }
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
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::While)) => {
                    Ok(Some(WhileToken { token_idx }))
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

impl<'a, Context> parsec::ParseFrom<Context> for BasicStmtKeywordToken
where
    Context: TokenParseContext<'a>,
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
                Token::Keyword(Keyword::Stmt(StmtKeyword::Assert)) => {
                    Ok(Some(AssertToken { token_idx }.into()))
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
                Token::Err(_)
                | Token::AuxiliaryIdentifier(_)
                | Token::Keyword(_)
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MatchToken {
    token_idx: TokenIdx,
}

impl MatchToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for MatchToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::Match)) => {
                    Ok(Some(MatchToken { token_idx }))
                }
                _ => Ok(None),
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
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Pattern(PatternKeyword::Mut)) => {
                    Ok(Some(MutToken { token_idx }))
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
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::If)) => Ok(Some(IfToken { token_idx })),
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
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::Elif)) => {
                    Ok(Some(ElifToken { token_idx }))
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
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::Else)) => {
                    Ok(Some(ElseToken { token_idx }))
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

// impl

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
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Impl) => Ok(Some(ImplToken { token_idx })),
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

// pub

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PubToken {
    token_idx: TokenIdx,
}

impl PubToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for PubToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Attr(AttributeKeyword::Pub) => Ok(Some(PubToken { token_idx })),
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

// use

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UseToken {
    token_idx: TokenIdx,
}

impl UseToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for UseToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Use) => Ok(Some(UseToken { token_idx })),
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
fn use_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<UseToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "use").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// crate

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CrateToken {
    token_idx: TokenIdx,
}

impl CrateToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for CrateToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Pronoun(PronounKeyword::Crate)) => {
                    Ok(Some(CrateToken { token_idx }))
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
fn crate_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<CrateToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "crate").unwrap().is_some());
    assert!(t(&db, "use").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

// self value

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SelfValueToken {
    token_idx: TokenIdx,
}

impl SelfValueToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for SelfValueToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Pronoun(PronounKeyword::SelfValue)) => {
                    Ok(Some(SelfValueToken { token_idx }))
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
fn self_value_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<SelfValueToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "self").unwrap().is_some());
    assert!(t(&db, "use").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

/// `Self` self type token
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SelfTypeToken {
    token_idx: TokenIdx,
}

impl SelfTypeToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for SelfTypeToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Pronoun(PronounKeyword::SelfType)) => {
                    Ok(Some(SelfTypeToken { token_idx }))
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
fn self_type_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<SelfTypeToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "Self").unwrap().is_some());
    assert!(t(&db, "use").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

/// `super` super token
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SuperToken {
    token_idx: TokenIdx,
}

impl SuperToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for SuperToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Pronoun(PronounKeyword::Super)) => {
                    Ok(Some(SuperToken { token_idx }))
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
fn super_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<SuperToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "super").unwrap().is_some());
    assert!(t(&db, "Self").unwrap().is_none());
    assert!(t(&db, "use").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarianceToken {
    Covariant(CovariantToken),
    Contravariant(ContravariantToken),
    Invariant(InvariantToken),
}

impl From<CovariantToken> for VarianceToken {
    fn from(v: CovariantToken) -> Self {
        Self::Covariant(v)
    }
}

impl From<ContravariantToken> for VarianceToken {
    fn from(v: ContravariantToken) -> Self {
        Self::Contravariant(v)
    }
}

impl From<InvariantToken> for VarianceToken {
    fn from(v: InvariantToken) -> Self {
        Self::Invariant(v)
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for VarianceToken
where
    Context: TokenParseContext<'a>,
{
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> Result<Option<Self>, <Context as HasParseError>::Error> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Pattern(PatternKeyword::Covariant)) => {
                    Ok(Some(CovariantToken { token_idx }.into()))
                }
                Token::Keyword(Keyword::Pattern(PatternKeyword::Contravariant)) => {
                    Ok(Some(ContravariantToken { token_idx }.into()))
                }
                Token::Keyword(Keyword::Pattern(PatternKeyword::Invariant)) => {
                    Ok(Some(InvariantToken { token_idx }.into()))
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
fn variance_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<VarianceToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "covariant").unwrap().is_some());
    assert!(t(&db, "contravariant").unwrap().is_some());
    assert!(t(&db, "invariant").unwrap().is_some());
    assert!(t(&db, "super").unwrap().is_none());
    assert!(t(&db, "Self").unwrap().is_none());
    assert!(t(&db, "use").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CovariantToken {
    token_idx: TokenIdx,
}

impl CovariantToken {
    pub fn token_idx(self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContravariantToken {
    token_idx: TokenIdx,
}

impl ContravariantToken {
    pub fn token_idx(self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvariantToken {
    token_idx: TokenIdx,
}

impl InvariantToken {
    pub fn token_idx(self) -> TokenIdx {
        self.token_idx
    }
}
