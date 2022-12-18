mod ambiguous;
mod config;
mod end;
mod form;
mod liason;
mod stmt;
mod ty;

pub use ambiguous::*;
pub use config::*;
pub use end::*;
pub use form::*;
pub use liason::*;
pub use stmt::*;
pub use ty::*;

use crate::*;

impl const From<Keyword> for TokenKind {
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
    Paradigm(FormKeyword),
    Type(TypeKeyword),
    Stmt(StmtKeyword),
    Liason(LiasonKeyword),
    Main,
    Use,
    Mod,
    Visual,
    Impl,
    Trait,
    End(EndKeyword),
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Keyword {
    pub const fn as_str(&self) -> &'static str {
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
            Keyword::Trait => "trait",
            Keyword::Impl => "impl",
            Keyword::End(_) => todo!(),
        }
    }
}

impl Deref for Keyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl const Into<TokenKind> for ConfigKeyword {
    fn into(self) -> TokenKind {
        TokenKind::Keyword(self.into())
    }
}
