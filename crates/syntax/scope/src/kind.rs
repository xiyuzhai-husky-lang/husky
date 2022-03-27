use token::{Special, Token, TokenKind};
use word::{Keyword, TyKeyword};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeKind {
    Module,
    Type(TyKind),
    Trait,
    Routine,
    Feature,
    Pattern,
    Literal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TyKind {
    Enum,
    Record,
    Struct,
    Primitive,
    Other,
}

impl From<TyKeyword> for TyKind {
    fn from(keyword: TyKeyword) -> Self {
        match keyword {
            TyKeyword::Struct => TyKind::Struct,
            TyKeyword::Rename => todo!(),
            TyKeyword::Enum => TyKind::Enum,
            TyKeyword::Props => todo!(),
            TyKeyword::Record => TyKind::Record,
        }
    }
}

impl ScopeKind {
    pub fn new(keyword: Keyword, third_token: &Token) -> Option<ScopeKind> {
        match keyword {
            Keyword::Use | Keyword::Stmt(_) | Keyword::Config(_) => None,
            Keyword::Mod => Some(ScopeKind::Module),
            Keyword::Routine(_) => Some(ScopeKind::Routine),
            Keyword::Type(keyword) => Some(ScopeKind::Type(keyword.into())),
            Keyword::Def => Some(match third_token.kind {
                TokenKind::Special(Special::LCurl) => ScopeKind::Pattern,
                _ => ScopeKind::Feature,
            }),
            Keyword::Main => todo!(),
        }
    }
}
