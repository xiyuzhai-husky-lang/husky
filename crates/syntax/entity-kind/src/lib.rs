use token::{Special, Token, TokenKind};
use word::{Keyword, TyKeyword};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TyKind {
    Enum,
    Record,
    Struct,
    Primitive,
    Vec,
    Array,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityKind {
    Module,
    Type(TyKind),
    Trait,
    TypeMember,
    Routine,
    Feature,
    Pattern,
    Literal,
    Member,
}

impl EntityKind {
    pub fn new(keyword: Keyword, third_token: &Token) -> Option<EntityKind> {
        match keyword {
            Keyword::Use | Keyword::Stmt(_) | Keyword::Config(_) => None,
            Keyword::Mod => Some(EntityKind::Module),
            Keyword::Routine(_) => Some(EntityKind::Routine),
            Keyword::Type(keyword) => Some(EntityKind::Type(keyword.into())),
            Keyword::Def => Some(match third_token.kind {
                TokenKind::Special(Special::LCurl) => EntityKind::Pattern,
                _ => EntityKind::Feature,
            }),
            Keyword::Main => todo!(),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnumVariantKind {
    Constant,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RoutineContextKind {
    Test,
    Proc,
    Func,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutineKind {
    Normal,
    TypeCall,
    Method,
    TypeAssociated,
    TraitAssociated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawMembRoutineKind {
    Proc,
    Func,
}

pub enum MemberKind {
    Method,
    Call,
    AssociatedType,
    AssociatedConstSize,
}
