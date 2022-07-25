mod decorator;
mod intern;
mod keyword;
mod opr;
mod style;

pub use decorator::*;
pub use ident::*;
pub use intern::{
    intern_word, new_word_interner, InternWord, WordInterner, WordInternerSingletonKeeper,
};
pub use keyword::*;
pub use opr::*;
pub use style::*;
pub type IdentDict<T> = VecMap<CustomIdentifier, T>;
pub type IdentArcDict<T> = VecMap<CustomIdentifier, Arc<T>>;
pub type IdentPairDict<T> = VecPairMap<CustomIdentifier, T>;

use std::sync::Arc;
use vec_like::{VecMap, VecPairMap};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum WordPtr {
    Keyword(Keyword),
    Identifier(Identifier),
    RawOpnVariant(WordOpr),
    Decorator(Decorator),
}

impl WordPtr {
    pub fn opt_ident(self) -> Option<Identifier> {
        match self {
            WordPtr::Identifier(ident) => Some(ident),
            _ => None,
        }
    }
    pub fn ident(self) -> Identifier {
        self.opt_ident().unwrap()
    }

    pub fn opt_custom(self) -> Option<CustomIdentifier> {
        self.opt_ident()
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

impl From<Paradigm> for WordPtr {
    fn from(func: Paradigm) -> Self {
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
