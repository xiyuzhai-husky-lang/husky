mod intern;
mod keyword;

pub use ident::{default_func_type, BuiltinIdentifier, CustomIdentifier, Identifier};
pub use intern::{convert_ident, new_word_interner, InternWord, WordInterner};
pub use keyword::{FuncKeyword, Keyword, StmtKeyword, TypeKeyword};

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

    pub fn user_defined_ident(self) -> Option<CustomIdentifier> {
        self.ident()
            .map(|ident| match ident {
                Identifier::Builtin(_) => None,
                Identifier::Custom(ident) => Some(ident),
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

impl From<TypeKeyword> for Word {
    fn from(Type: TypeKeyword) -> Self {
        Self::Keyword(Type.into())
    }
}

impl From<FuncKeyword> for Word {
    fn from(Func: FuncKeyword) -> Self {
        Self::Keyword(Func.into())
    }
}

impl From<StmtKeyword> for Word {
    fn from(stmt: StmtKeyword) -> Self {
        Self::Keyword(stmt.into())
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
