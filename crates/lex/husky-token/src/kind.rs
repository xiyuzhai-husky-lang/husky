mod decorator;
mod keyword;
mod special;
mod wordopr;

pub use decorator::*;
pub use keyword::*;
pub use special::*;
pub use wordopr::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Attr(AttrKeyword),
    Keyword(Keyword),
    Identifier(Identifier),
    Special(SpecialToken),
    WordOpr(WordOpr),
    Literal(LiteralToken),
    Unrecognized(char),
    IllFormedLiteral(LiteralToken),
    Comment,
}

impl std::hash::Hash for TokenKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
