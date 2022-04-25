use vm::PrimitiveValue;
use word::{Identifier, Keyword, WordPtr};

pub use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Keyword(Keyword),
    Identifier(Identifier),
    Special(Special),
    PrimitiveLiteral(PrimitiveValue),
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
        }
    }
}
