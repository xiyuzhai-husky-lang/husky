mod intern;
mod keyword;

pub use ident::{
    default_func_type, BuiltinIdentifier, CustomIdentifier, Identifier, ImplicitIdentifier,
};
pub use intern::{new_word_unique_allocator, InternWord, WordInterner};
pub use keyword::{ConfigKeyword, FuncKeyword, Keyword, StmtKeyword, TypeKeyword};

use common::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum WordPtr {
    Keyword(Keyword),
    Identifier(Identifier),
}

impl WordPtr {
    pub fn ident(self) -> Option<Identifier> {
        match self {
            WordPtr::Keyword(_) => None,
            WordPtr::Identifier(ident) => Some(ident),
        }
    }

    pub fn custom(self) -> Option<CustomIdentifier> {
        self.ident()
            .map(|ident| match ident {
                Identifier::Builtin(_) | Identifier::Implicit(_) => None,
                Identifier::Custom(ident) => Some(ident),
            })
            .flatten()
    }
}

impl From<Keyword> for WordPtr {
    fn from(keyword: Keyword) -> Self {
        Self::Keyword(keyword)
    }
}

impl From<TypeKeyword> for WordPtr {
    fn from(ty: TypeKeyword) -> Self {
        Self::Keyword(ty.into())
    }
}

impl From<ConfigKeyword> for WordPtr {
    fn from(func: ConfigKeyword) -> Self {
        Self::Keyword(func.into())
    }
}

impl From<FuncKeyword> for WordPtr {
    fn from(func: FuncKeyword) -> Self {
        Self::Keyword(func.into())
    }
}

impl From<StmtKeyword> for WordPtr {
    fn from(stmt: StmtKeyword) -> Self {
        Self::Keyword(stmt.into())
    }
}

impl From<Identifier> for WordPtr {
    fn from(ident: Identifier) -> Self {
        Self::Identifier(ident)
    }
}

impl From<BuiltinIdentifier> for WordPtr {
    fn from(reserved: BuiltinIdentifier) -> Self {
        WordPtr::Identifier(Identifier::Builtin(reserved))
    }
}

impl From<CustomIdentifier> for WordPtr {
    fn from(ident: CustomIdentifier) -> Self {
        WordPtr::Identifier(Identifier::Custom(ident))
    }
}

impl From<ImplicitIdentifier> for WordPtr {
    fn from(ident: ImplicitIdentifier) -> Self {
        WordPtr::Identifier(Identifier::Implicit(ident))
    }
}

mod ident;
