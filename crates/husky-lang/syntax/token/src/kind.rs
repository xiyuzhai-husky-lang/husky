pub use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Keyword(word::Keyword),
    Identifier(word::Identifier),
    Special(Special),
    I32Literal(i32),
    F32Literal(f32),
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
impl From<word::WordId> for TokenKind {
    fn from(word: word::WordId) -> Self {
        match word {
            word::WordId::Keyword(keyword) => TokenKind::Keyword(keyword),
            word::WordId::Identifier(ident) => TokenKind::Identifier(ident),
        }
    }
}
