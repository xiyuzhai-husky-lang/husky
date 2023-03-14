mod attribute;
mod keyword;
mod punctuation;
mod wordopr;

pub use attribute::*;
use husky_word::Label;
pub use keyword::*;
pub use punctuation::*;
pub use wordopr::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub enum Token {
    Attr(AttributeKeyword),
    Keyword(Keyword),
    Ident(Ident),
    Label(Label),
    Punctuation(Punctuation),
    WordOpr(WordOpr),
    Literal(Literal),
    Error(TokenError),
}

impl std::hash::Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
