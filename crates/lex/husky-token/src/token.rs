mod keyword;
mod literal;
mod punctuation;
mod wordopr;

pub use self::keyword::*;
pub use self::literal::*;
pub use self::punctuation::*;
pub use self::wordopr::*;

use crate::*;
use husky_coword::Label;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenDb, jar = TokenJar)]
#[enum_class::from_variants]
pub enum TokenData {
    Keyword(Keyword),
    Ident(Ident),
    Label(Label),
    Punctuation(Punctuation),
    WordOpr(WordOpr),
    Literal(Literal),
    Error(TokenDataError),
}

impl std::hash::Hash for TokenData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
