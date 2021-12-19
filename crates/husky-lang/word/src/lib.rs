mod intern;
mod keyword;

pub use ident::{Identifier, Reserved};
pub use intern::{convert_ident, new_word_interner, InternWord, WordInterner};
pub use keyword::Keyword;

use common::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Word {
    Keyword(Keyword),
    Identifier(Identifier),
}

impl Word {
    pub fn ident(self) -> Option<Identifier> {
        match self {
            Word::Keyword(_) => None,
            Word::Identifier(ident) => Some(ident),
        }
    }
}

impl From<u32> for Word {
    fn from(raw: u32) -> Self {
        Word::Identifier(raw.into())
    }
}

impl From<Keyword> for Word {
    fn from(keyword: Keyword) -> Self {
        Self::Keyword(keyword)
    }
}

impl From<Identifier> for Word {
    fn from(ident: Identifier) -> Self {
        Self::Identifier(ident)
    }
}

impl From<Reserved> for Word {
    fn from(reserved: Reserved) -> Self {
        Word::Identifier(Identifier::Reserved(reserved))
    }
}

mod ident;

pub fn use_string<F, Q>(this: &(impl InternWord + ?Sized), word: Word, f: F) -> Q
where
    F: Fn(&str) -> Q,
{
    this.word_interner().apply(word, f)
}
