mod intern;
mod keyword;

pub use ident::{default_func_type, CustomIdentifier, Identifier, ReservedIdentifier};
pub use intern::{new_word_interner, InternWord, WordInterner};
pub use keyword::{ConfigKeyword, FuncKeyword, Keyword, StmtKeyword, TypeKeyword};

use common::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum WordId {
    Keyword(Keyword),
    Identifier(Identifier),
}

impl WordId {
    pub fn ident(self) -> Option<Identifier> {
        match self {
            WordId::Keyword(_) => None,
            WordId::Identifier(ident) => Some(ident),
        }
    }

    pub fn custom(self) -> Option<CustomIdentifier> {
        self.ident()
            .map(|ident| match ident {
                Identifier::Builtin(_) => None,
                Identifier::Custom(ident) => Some(ident),
            })
            .flatten()
    }
}

impl From<Keyword> for WordId {
    fn from(keyword: Keyword) -> Self {
        Self::Keyword(keyword)
    }
}

impl From<TypeKeyword> for WordId {
    fn from(ty: TypeKeyword) -> Self {
        Self::Keyword(ty.into())
    }
}

impl From<ConfigKeyword> for WordId {
    fn from(func: ConfigKeyword) -> Self {
        Self::Keyword(func.into())
    }
}

impl From<FuncKeyword> for WordId {
    fn from(func: FuncKeyword) -> Self {
        Self::Keyword(func.into())
    }
}

impl From<StmtKeyword> for WordId {
    fn from(stmt: StmtKeyword) -> Self {
        Self::Keyword(stmt.into())
    }
}

impl From<Identifier> for WordId {
    fn from(ident: Identifier) -> Self {
        Self::Identifier(ident)
    }
}

impl From<ReservedIdentifier> for WordId {
    fn from(reserved: ReservedIdentifier) -> Self {
        WordId::Identifier(Identifier::Builtin(reserved))
    }
}

impl From<CustomIdentifier> for WordId {
    fn from(ident: CustomIdentifier) -> Self {
        WordId::Identifier(Identifier::Custom(ident))
    }
}

mod ident;
