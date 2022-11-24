mod ambiguous;
mod config;
mod liason;
mod paradigm;
mod stmt;

pub use ambiguous::*;
pub use config::*;
pub use liason::*;
pub use paradigm::*;
pub use stmt::*;

use crate::TokenKind;

impl From<Keyword> for TokenKind {
    fn from(keyword: Keyword) -> Self {
        TokenKind::Keyword(keyword)
    }
}

pub use liason::*;

use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Keyword {
    Config(ConfigKeyword),
    Paradigm(Paradigm),
    Type(TyKeyword),
    Stmt(StmtKeyword),
    Liason(LiasonKeyword),
    Ambiguous(AmbiguousKeyword),
    Main,
    Use,
    Mod,
    Visual,
    Impl,
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Keyword {
    pub fn as_str(&self) -> &'static str {
        match self {
            Keyword::Config(keyword) => keyword.as_str(),
            Keyword::Paradigm(keyword) => keyword.as_str(),
            Keyword::Type(keyword) => keyword.as_str(),
            Keyword::Stmt(keyword) => keyword.as_str(),
            Keyword::Use => "use",
            Keyword::Mod => "mod",
            Keyword::Main => "main",
            Keyword::Visual => "visual",
            Keyword::Liason(keyword) => keyword.as_str(),
            Keyword::Impl => "impl",
            Keyword::Ambiguous(_) => todo!(),
        }
    }
}

impl Deref for Keyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl From<Paradigm> for Keyword {
    fn from(kw: Paradigm) -> Self {
        Keyword::Paradigm(kw)
    }
}

impl From<TyKeyword> for Keyword {
    fn from(kw: TyKeyword) -> Self {
        Keyword::Type(kw)
    }
}

impl const Into<TokenKind> for ConfigKeyword {
    fn into(self) -> TokenKind {
        TokenKind::Keyword(self.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum Paradigm {
    LazyFunctional,
    EagerFunctional,
    EagerProcedural,
}

impl Paradigm {
    pub fn as_str(self) -> &'static str {
        match self {
            Paradigm::EagerProcedural => "proc",
            Paradigm::EagerFunctional => "func",
            Paradigm::LazyFunctional => "def",
        }
    }

    pub fn is_lazy(self) -> bool {
        match self {
            Paradigm::LazyFunctional => true,
            Paradigm::EagerFunctional | Paradigm::EagerProcedural => false,
        }
    }
}

impl Deref for Paradigm {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TyKeyword {
    Struct,
    Enum,
    Record,
}

impl TyKeyword {
    fn as_str(&self) -> &'static str {
        match self {
            TyKeyword::Struct => "struct",
            TyKeyword::Enum => "enum",
            TyKeyword::Record => "record",
        }
    }
}

impl Deref for TyKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
