mod liason;

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
    Main,
    Use,
    Mod,
    Visual,
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
        }
    }
}

impl Deref for Keyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl From<ConfigKeyword> for Keyword {
    fn from(kw: ConfigKeyword) -> Self {
        Self::Config(kw)
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

impl From<StmtKeyword> for Keyword {
    fn from(stmt: StmtKeyword) -> Self {
        Keyword::Stmt(stmt)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ConfigKeyword {
    Task,
}

impl ConfigKeyword {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfigKeyword::Task => "task",
        }
    }
}

impl Deref for ConfigKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum StmtKeyword {
    Let,
    Var,
    If,
    Elif,
    Else,
    Match,
    Case,
    DeFault,
    For,
    ForExt,
    While,
    Do,
    Break,
    Return,
    Assert,
}

impl StmtKeyword {
    pub fn as_str(&self) -> &'static str {
        match self {
            StmtKeyword::Let => "let",
            StmtKeyword::Var => "var",
            StmtKeyword::If => "if",
            StmtKeyword::Elif => "elif",
            StmtKeyword::Else => "else",
            StmtKeyword::Match => "match",
            StmtKeyword::Case => "case",
            StmtKeyword::DeFault => "default",
            StmtKeyword::For => "for",
            StmtKeyword::ForExt => "forext",
            StmtKeyword::While => "while",
            StmtKeyword::Do => "do",
            StmtKeyword::Break => "break",
            StmtKeyword::Return => "return",
            StmtKeyword::Assert => "assert",
        }
    }
}

impl Deref for StmtKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
