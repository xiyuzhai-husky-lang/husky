use vm::CopyableValue;
use word::{Decorator, Identifier, Keyword, WordOpr, WordPtr};

pub use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Decorator(Decorator),
    Keyword(Keyword),
    Identifier(Identifier),
    Special(Special),
    WordOpr(WordOpr),
    PrimitiveLiteral(CopyableValue),
    Unrecognized(char),
    IllFormedLiteral(CopyableValue),
}

impl From<Special> for TokenKind {
    fn from(special: Special) -> Self {
        TokenKind::Special(special)
    }
}
impl std::hash::Hash for TokenKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
impl Eq for TokenKind {}
impl From<WordPtr> for TokenKind {
    fn from(word: WordPtr) -> Self {
        match word {
            WordPtr::Keyword(keyword) => TokenKind::Keyword(keyword),
            WordPtr::Identifier(ident) => TokenKind::Identifier(ident),
            WordPtr::RawOpnVariant(word_opr) => TokenKind::WordOpr(word_opr),
            WordPtr::Decorator(decorator) => TokenKind::Decorator(decorator),
        }
    }
}
