pub use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Keyword(word::Keyword),
    Identifier(word::Identifier),
    Special(Special),
    IntegerLiteral(String),
    FloatLiteral(String),
}
impl std::hash::Hash for TokenKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
impl Eq for TokenKind {}
impl From<word::Word> for TokenKind {
    fn from(word: word::Word) -> Self {
        match word {
            word::Word::Keyword(keyword) => TokenKind::Keyword(keyword),
            word::Word::Identifier(ident) => TokenKind::Identifier(ident),
        }
    }
}
