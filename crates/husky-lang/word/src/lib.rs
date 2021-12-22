mod intern;
mod keyword;

pub use ident::{default_routine_type, BuiltinIdentifier, Identifier, UserDefinedIdentifier};
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

    pub fn user_defined_ident(self) -> Option<UserDefinedIdentifier> {
        self.ident()
            .map(|ident| match ident {
                Identifier::Builtin(_) => None,
                Identifier::UserDefined(ident) => Some(ident),
            })
            .flatten()
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

impl From<BuiltinIdentifier> for Word {
    fn from(reserved: BuiltinIdentifier) -> Self {
        Word::Identifier(Identifier::Builtin(reserved))
    }
}

mod ident;

pub fn use_string<F, Q>(this: &(impl InternWord + ?Sized), word: Word, f: F) -> Q
where
    F: Fn(&str) -> Q,
{
    this.word_interner().apply(word, f)
}
