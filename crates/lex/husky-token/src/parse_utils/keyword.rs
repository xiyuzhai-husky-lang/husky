mod entity_kind;
mod symbol_modifier;

pub use self::entity_kind::*;
pub use self::symbol_modifier::*;

use husky_entity_taxonomy::TypeKind;

use super::*;

// let

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct LetToken {
    token_idx: TokenIdx,
}

impl LetToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct ReturnToken {
    token_idx: TokenIdx,
}

impl ReturnToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct RequireToken {
    token_idx: TokenIdx,
}

impl RequireToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct AssertToken {
    token_idx: TokenIdx,
}

impl AssertToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct BreakToken {
    token_idx: TokenIdx,
}

impl BreakToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct StmtForToken {
    token_idx: TokenIdx,
}

impl StmtForToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct ForextToken {
    token_idx: TokenIdx,
}

impl ForextToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct WhileToken {
    token_idx: TokenIdx,
}

impl WhileToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct DoToken {
    token_idx: TokenIdx,
}

impl DoToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
#[enum_class::from_variants]
pub enum BasicStmtKeywordToken {
    Let(LetToken),
    Return(ReturnToken),
    Require(RequireToken),
    Assert(AssertToken),
    Break(BreakToken),
    For(StmtForToken),
    ForExt(ForextToken),
    While(WhileToken),
    Do(DoToken),
}

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for WhileToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::While)) => {
                    Ok(Some(WhileToken { token_idx }))
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

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for BasicStmtKeywordToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Stmt(stmt_keyword)) => match stmt_keyword {
                    StmtKeyword::Let => Ok(Some(LetToken { token_idx }.into())),
                    StmtKeyword::Return => Ok(Some(ReturnToken { token_idx }.into())),
                    StmtKeyword::Require => Ok(Some(RequireToken { token_idx }.into())),
                    StmtKeyword::Assert => Ok(Some(AssertToken { token_idx }.into())),
                    StmtKeyword::Break => Ok(Some(BreakToken { token_idx }.into())),
                    StmtKeyword::NonImplFor => Ok(Some(StmtForToken { token_idx }.into())),
                    StmtKeyword::ForExt => Ok(Some(ForextToken { token_idx }.into())),
                    StmtKeyword::While => Ok(Some(WhileToken { token_idx }.into())),
                    StmtKeyword::Do => Ok(Some(DoToken { token_idx }.into())),
                    StmtKeyword::If => todo!(),
                    StmtKeyword::Elif => todo!(),
                    StmtKeyword::Else => todo!(),
                    StmtKeyword::Match => todo!(),
                },
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Keyword(_)
                | Token::Punctuation(_)
                | Token::Ident(_)
                | Token::WordOpr(_)
                | Token::Literal(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct MatchToken {
    token_idx: TokenIdx,
}

impl MatchToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for MatchToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
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

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct IfToken {
    token_idx: TokenIdx,
}

impl IfToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for IfToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::If)) => Ok(Some(IfToken { token_idx })),
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

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct ElifToken {
    token_idx: TokenIdx,
}

impl ElifToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for ElifToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::Elif)) => {
                    Ok(Some(ElifToken { token_idx }))
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

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct ElseToken {
    token_idx: TokenIdx,
}

impl ElseToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for ElseToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Stmt(StmtKeyword::Else)) => {
                    Ok(Some(ElseToken { token_idx }))
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

// impl

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct ImplToken {
    token_idx: TokenIdx,
}

impl ImplToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for ImplToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Impl) => Ok(Some(ImplToken { token_idx })),
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

// pub

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct PubToken {
    token_idx: TokenIdx,
}

impl PubToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for PubToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Pub) => Ok(Some(PubToken { token_idx })),
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

// use

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct UseToken {
    token_idx: TokenIdx,
}

impl UseToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for UseToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Use) => Ok(Some(UseToken { token_idx })),
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

// self value

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct SelfValueToken {
    token_idx: TokenIdx,
}

impl SelfValueToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for SelfValueToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Pronoun(PronounKeyword::SelfValue)) => {
                    Ok(Some(SelfValueToken { token_idx }))
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
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct SelfTypeToken {
    token_idx: TokenIdx,
}

impl SelfTypeToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for SelfTypeToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Pronoun(PronounKeyword::SelfType)) => {
                    Ok(Some(SelfTypeToken { token_idx }))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
#[enum_class::from_variants]
pub enum VarianceToken {
    Covariant(CovariantToken),
    Contravariant(ContravariantToken),
    Invariant(InvariantToken),
}

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for VarianceToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Modifier(ModifierKeyword::Covariant)) => {
                    Ok(Some(CovariantToken { token_idx }.into()))
                }
                Token::Keyword(Keyword::Modifier(ModifierKeyword::Contravariant)) => {
                    Ok(Some(ContravariantToken { token_idx }.into()))
                }
                Token::Keyword(Keyword::Modifier(ModifierKeyword::Invariant)) => {
                    Ok(Some(InvariantToken { token_idx }.into()))
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
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct CovariantToken {
    token_idx: TokenIdx,
}

impl CovariantToken {
    pub fn token_idx(self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct ContravariantToken {
    token_idx: TokenIdx,
}

impl ContravariantToken {
    pub fn token_idx(self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct InvariantToken {
    token_idx: TokenIdx,
}

impl InvariantToken {
    pub fn token_idx(self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConnectionForToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::TryParseOptionalFromStream<Context> for ConnectionForToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Keyword(Keyword::Connection(ConnectionKeyword::For)) => {
                    Ok(Some(ConnectionForToken { token_idx }))
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
