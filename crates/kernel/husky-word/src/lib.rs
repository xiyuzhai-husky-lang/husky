mod decorator;
mod intern;
mod keyword;
mod menu;
mod opr;
mod pattern;
mod style;

pub use decorator::*;
pub use ident::*;
pub use intern::{InternWord, WordInterner};
pub use keyword::*;
pub use opr::*;
pub use pattern::*;
pub use style::*;
pub type IdentDict<T> = VecMap<CustomIdentifier, T>;
pub type IdentArcDict<T> = VecMap<CustomIdentifier, Arc<T>>;
pub type IdentPairDict<T> = VecPairMap<CustomIdentifier, T>;

use std::sync::Arc;
use vec_like::{VecMap, VecPairMap};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum WordItd {
    Keyword(Keyword),
    Identifier(Identifier),
    Opr(WordOpr),
    Decorator(Decorator),
    Pattern(WordPattern),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Word(String);

impl Word {
    pub fn new(s: String) -> Self {
        todo!()
    }
}

impl WordItd {
    pub fn opt_ident(self) -> Option<Identifier> {
        match self {
            WordItd::Identifier(ident) => Some(ident),
            _ => None,
        }
    }

    pub fn ident(self) -> Identifier {
        self.opt_ident().unwrap()
    }

    pub fn opt_custom(self) -> Option<CustomIdentifier> {
        self.opt_ident()
            .map(|ident| match ident {
                Identifier::Root(_) | Identifier::Contextual(_) => None,
                Identifier::Custom(ident) => Some(ident),
            })
            .flatten()
    }

    pub fn custom(self) -> CustomIdentifier {
        self.opt_custom().unwrap()
    }
}

impl From<Keyword> for WordItd {
    fn from(keyword: Keyword) -> Self {
        Self::Keyword(keyword)
    }
}

impl From<TyKeyword> for WordItd {
    fn from(ty: TyKeyword) -> Self {
        Self::Keyword(ty.into())
    }
}

impl From<ConfigKeyword> for WordItd {
    fn from(func: ConfigKeyword) -> Self {
        Self::Keyword(func.into())
    }
}

impl From<Paradigm> for WordItd {
    fn from(func: Paradigm) -> Self {
        Self::Keyword(func.into())
    }
}

impl From<StmtKeyword> for WordItd {
    fn from(stmt: StmtKeyword) -> Self {
        Self::Keyword(stmt.into())
    }
}

impl From<Identifier> for WordItd {
    fn from(ident: Identifier) -> Self {
        Self::Identifier(ident)
    }
}

impl From<RootBuiltinIdentifier> for WordItd {
    fn from(ident: RootBuiltinIdentifier) -> Self {
        WordItd::Identifier(Identifier::Root(ident))
    }
}

impl From<CustomIdentifier> for WordItd {
    fn from(ident: CustomIdentifier) -> Self {
        WordItd::Identifier(Identifier::Custom(ident))
    }
}

impl From<ContextualIdentifier> for WordItd {
    fn from(ident: ContextualIdentifier) -> Self {
        WordItd::Identifier(Identifier::Contextual(ident))
    }
}

mod ident;
