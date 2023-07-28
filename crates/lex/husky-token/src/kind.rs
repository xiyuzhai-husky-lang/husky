mod keyword;
mod punctuation;
mod wordopr;

pub use self::keyword::*;
pub use self::punctuation::*;
pub use self::wordopr::*;

use crate::*;
use husky_coword::Label;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenDb)]
#[enum_class::from_variants]
pub enum Token {
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
