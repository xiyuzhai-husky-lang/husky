mod ambiguous;
mod config;
mod end;
mod form;
mod modifier;
mod pronoun;
mod stmt;
mod ty;

pub use ambiguous::*;
pub use config::*;
pub use end::*;
pub use form::*;
pub use modifier::*;
pub use pronoun::*;
pub use stmt::*;
pub use ty::*;

use crate::*;

pub use modifier::*;

use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = TokenDb)]
#[enum_class::from_variants]
pub enum Keyword {
    Config(ConfigKeyword),
    Fugitive(FugitiveKeyword),
    TypeEntity(TypeEntityKeyword),
    Stmt(StmtKeyword),
    Modifier(ModifierKeyword),
    Pronoun(PronounKeyword),
    Main,
    Use,
    Mod,
    Impl,
    Trait,
    Connection(ConnectionKeyword),
    End(EndKeyword),
    Pub,
    Const,
    Static,
    Async,
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
            Keyword::Fugitive(keyword) => keyword.code(),
            Keyword::TypeEntity(keyword) => keyword.code(),
            Keyword::Stmt(keyword) => keyword.code(),
            Keyword::Use => "use",
            Keyword::Mod => "mod",
            Keyword::Main => "main",
            Keyword::Modifier(keyword) => keyword.code(),
            Keyword::Trait => "trait",
            Keyword::Impl => "impl",
            Keyword::End(_) => todo!(),
            Keyword::Connection(_) => todo!(),
            Keyword::Pronoun(_) => todo!(),
            Keyword::Pub => "pub",
            Keyword::Const => "const",
            Keyword::Static => "static",
            Keyword::Async => "async",
        }
    }
}

impl Deref for Keyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}

impl From<ConfigKeyword> for Token {
    fn from(val: ConfigKeyword) -> Self {
        Token::Keyword(val.into())
    }
}
