use vm::CopyableValue;
use word::{Decorator, Identifier, Keyword, WordOpr, WordPtr};

pub use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub enum HuskyTokenKind {
    Decorator(Decorator),
    Keyword(Keyword),
    Identifier(Identifier),
    Special(SpecialToken),
    WordOpr(WordOpr),
    PrimitiveLiteral(CopyableValue),
    Unrecognized(char),
    IllFormedLiteral(CopyableValue),
}

impl From<SpecialToken> for HuskyTokenKind {
    fn from(special: SpecialToken) -> Self {
        HuskyTokenKind::Special(special)
    }
}
impl std::hash::Hash for HuskyTokenKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
impl Eq for HuskyTokenKind {}
impl From<WordPtr> for HuskyTokenKind {
    fn from(word: WordPtr) -> Self {
        match word {
            WordPtr::Keyword(keyword) => HuskyTokenKind::Keyword(keyword),
            WordPtr::Identifier(ident) => HuskyTokenKind::Identifier(ident),
            WordPtr::RawOpnVariant(word_opr) => HuskyTokenKind::WordOpr(word_opr),
            WordPtr::Decorator(decorator) => HuskyTokenKind::Decorator(decorator),
        }
    }
}

impl From<Keyword> for HuskyTokenKind {
    fn from(keyword: Keyword) -> Self {
        HuskyTokenKind::Keyword(keyword)
    }
}
