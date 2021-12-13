pub use crate::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Keyword(word::Keyword),
    Identifier(word::Identifier),
    Special(Special),
    I32Literal(i32),
    F32Literal(f32),
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
impl From<f32> for TokenKind {
    fn from(f: f32) -> Self {
        TokenKind::F32Literal(f)
    }
}
impl From<i32> for TokenKind {
    fn from(i: i32) -> Self {
        TokenKind::I32Literal(i)
    }
}
