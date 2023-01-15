mod attribute;
mod keyword;
mod punctuation;
mod wordopr;

pub use attribute::*;
pub use keyword::*;
pub use punctuation::*;
pub use wordopr::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Attr(AttributeKeyword),
    Keyword(Keyword),
    Identifier(Identifier),
    Punctuation(Punctuation),
    WordOpr(WordOpr),
    Literal(Literal),
    Err(TokenError),
}

impl<Db: TokenDb + ?Sized> salsa::DebugWithDb<Db> for Token {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TokenJar>>::as_jar_db(db);
        match self {
            _ => <Self as std::fmt::Debug>::fmt(&self, f),
        }
    }
}

impl std::hash::Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
