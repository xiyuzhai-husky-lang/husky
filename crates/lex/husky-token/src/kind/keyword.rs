mod ambiguous;
mod config;
mod end;
mod form;
mod liason;
mod pronoun;
mod stmt;
mod ty;

pub use ambiguous::*;
pub use config::*;
pub use end::*;
pub use form::*;
pub use liason::*;
pub use pronoun::*;
pub use stmt::*;
pub use ty::*;

use crate::*;

impl const From<Keyword> for Token {
    fn from(keyword: Keyword) -> Self {
        Token::Keyword(keyword)
    }
}

pub use liason::*;

use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Keyword {
    Config(ConfigKeyword),
    Paradigm(FormKeyword),
    Type(TypeKeyword),
    Stmt(StmtKeyword),
    Liason(LiasonKeyword),
    Pronoun(PronounKeyword),
    Main,
    Use,
    Mod,
    Visual,
    Impl,
    Trait,
    Connection(ConnectionKeyword),
    End(EndKeyword),
}

impl From<ConnectionKeyword> for Keyword {
    fn from(v: ConnectionKeyword) -> Self {
        Self::Connection(v)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ConnectionKeyword {
    For,
    Where,
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.code().fmt(f)
    }
}

impl Keyword {
    pub const fn code(&self) -> &'static str {
        match self {
            Keyword::Config(keyword) => keyword.code(),
            Keyword::Paradigm(keyword) => keyword.code(),
            Keyword::Type(keyword) => keyword.code(),
            Keyword::Stmt(keyword) => keyword.code(),
            Keyword::Use => "use",
            Keyword::Mod => "mod",
            Keyword::Main => "main",
            Keyword::Visual => "visual",
            Keyword::Liason(keyword) => keyword.code(),
            Keyword::Trait => "trait",
            Keyword::Impl => "impl",
            Keyword::End(_) => todo!(),
            Keyword::Connection(_) => todo!(),
            Keyword::Pronoun(_) => todo!(),
        }
    }
}

impl Deref for Keyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}

impl const From<ConfigKeyword> for Token {
    fn from(val: ConfigKeyword) -> Self {
        Token::Keyword(val.into())
    }
}
