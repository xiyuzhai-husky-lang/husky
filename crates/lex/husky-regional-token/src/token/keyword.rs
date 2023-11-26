mod item_kind;
mod stmt;

pub use self::item_kind::*;
pub use self::stmt::*;

use husky_entity_kind::TypeKind;
use husky_term_prelude::Variance;

use super::*;

// impl

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenDb, jar = TokenJar)]
pub struct ImplRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl ImplRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for ImplRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Impl) => {
                    Ok(Some(ImplRegionalToken { regional_token_idx }))
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

// pub

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenDb, jar = TokenJar)]
pub struct PubRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl PubRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for PubRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Pub) => {
                    Ok(Some(PubRegionalToken { regional_token_idx }))
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

// use

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenDb, jar = TokenJar)]
pub struct UseRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl UseRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for UseRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Use) => {
                    Ok(Some(UseRegionalToken { regional_token_idx }))
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

#[test]
fn use_token_works() {
    fn t(db: &::salsa::Db, input: &str) -> TokenDataResult<Option<UseRegionalToken>> {
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
#[salsa::debug_with_db(db = TokenDb, jar = TokenJar)]
pub struct SelfValueRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl SelfValueRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for SelfValueRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Pronoun(PronounKeyword::SelfValue)) => {
                    Ok(Some(SelfValueRegionalToken { regional_token_idx }))
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

#[test]
fn self_value_token_works() {
    fn t(db: &::salsa::Db, input: &str) -> TokenDataResult<Option<SelfValueRegionalToken>> {
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
#[salsa::debug_with_db(db = TokenDb, jar = TokenJar)]
pub struct SelfTypeRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl SelfTypeRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for SelfTypeRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Pronoun(PronounKeyword::SelfType)) => {
                    Ok(Some(SelfTypeRegionalToken { regional_token_idx }))
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

#[test]
fn self_type_token_works() {
    fn t(db: &::salsa::Db, input: &str) -> TokenDataResult<Option<SelfTypeRegionalToken>> {
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
#[salsa::debug_with_db(db = TokenDb, jar = TokenJar)]
#[enum_class::from_variants]
pub enum VarianceRegionalToken {
    Covariant(CovariantRegionalToken),
    Contravariant(ContravariantRegionalToken),
    Invariant(InvariantRegionalToken),
}

impl Into<Variance> for VarianceRegionalToken {
    fn into(self) -> Variance {
        match self {
            VarianceRegionalToken::Covariant(_) => Variance::Covariant,
            VarianceRegionalToken::Contravariant(_) => Variance::Contravariant,
            VarianceRegionalToken::Invariant(_) => Variance::Invariant,
        }
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for VarianceRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Modifier(ModifierKeyword::Covariant)) => {
                    Ok(Some(CovariantRegionalToken { regional_token_idx }.into()))
                }
                TokenData::Keyword(Keyword::Modifier(ModifierKeyword::Contravariant)) => Ok(Some(
                    ContravariantRegionalToken { regional_token_idx }.into(),
                )),
                TokenData::Keyword(Keyword::Modifier(ModifierKeyword::Invariant)) => {
                    Ok(Some(InvariantRegionalToken { regional_token_idx }.into()))
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

#[test]
fn variance_token_works() {
    fn t(db: &::salsa::Db, input: &str) -> TokenDataResult<Option<VarianceRegionalToken>> {
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
#[salsa::debug_with_db(db = TokenDb, jar = TokenJar)]
pub struct CovariantRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl CovariantRegionalToken {
    pub fn regional_token_idx(self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb, jar = TokenJar)]
pub struct ContravariantRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl ContravariantRegionalToken {
    pub fn regional_token_idx(self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenDb, jar = TokenJar)]
pub struct InvariantRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl InvariantRegionalToken {
    pub fn regional_token_idx(self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConnectionForRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for ConnectionForRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Connection(ConnectionKeyword::For)) => {
                    Ok(Some(ConnectionForRegionalToken { regional_token_idx }))
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
