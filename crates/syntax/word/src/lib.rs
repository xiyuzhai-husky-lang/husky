mod alloc;
mod keyword;
mod utils;

pub use alloc::{new_word_unique_allocator, InternWord, WordAllocator};
pub use ident::{
    default_func_type, ContextualIdentifier, CustomIdentifier, Identifier, RootIdentifier,
};
pub use keyword::{ConfigKeyword, Keyword, RoutineKeyword, StmtKeyword, TyKeyword};
pub use utils::*;

pub type IdentMap<T> = VecDict<CustomIdentifier, T>;

use vec_map::VecDict;

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

    pub fn opt_custom(self) -> Option<CustomIdentifier> {
        self.ident()
            .map(|ident| match ident {
                Identifier::Builtin(_) | Identifier::Contextual(_) => None,
                Identifier::Custom(ident) => Some(ident),
            })
            .flatten()
    }

    pub fn custom(self) -> CustomIdentifier {
        self.opt_custom().unwrap()
    }
}

impl From<Keyword> for WordPtr {
    fn from(keyword: Keyword) -> Self {
        Self::Keyword(keyword)
    }
}

impl From<TyKeyword> for WordPtr {
    fn from(ty: TyKeyword) -> Self {
        Self::Keyword(ty.into())
    }
}

impl From<ConfigKeyword> for WordPtr {
    fn from(func: ConfigKeyword) -> Self {
        Self::Keyword(func.into())
    }
}

impl From<RoutineKeyword> for WordPtr {
    fn from(func: RoutineKeyword) -> Self {
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

impl From<RootIdentifier> for WordPtr {
    fn from(ident: RootIdentifier) -> Self {
        WordPtr::Identifier(Identifier::Builtin(ident))
    }
}

impl From<CustomIdentifier> for WordPtr {
    fn from(ident: CustomIdentifier) -> Self {
        WordPtr::Identifier(Identifier::Custom(ident))
    }
}

impl From<ContextualIdentifier> for WordPtr {
    fn from(ident: ContextualIdentifier) -> Self {
        WordPtr::Identifier(Identifier::Contextual(ident))
    }
}

mod ident;
