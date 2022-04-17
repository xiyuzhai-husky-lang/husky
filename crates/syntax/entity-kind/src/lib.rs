use token::{Special, Token, TokenKind};
use word::{Keyword, TyKeyword};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeKind {
    Enum,
    Record,
    Struct,
    Primitive,
    Vec,
    Array,
    Other,
}

impl From<TyKeyword> for TypeKind {
    fn from(keyword: TyKeyword) -> Self {
        match keyword {
            TyKeyword::Struct => TypeKind::Struct,
            TyKeyword::Rename => todo!(),
            TyKeyword::Enum => TypeKind::Enum,
            TyKeyword::Props => todo!(),
            TyKeyword::Record => TypeKind::Record,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityKind {
    Module,
    Type(TypeKind),
    Trait,
    TypeMember,
    Routine,
    Feature,
    Pattern,
    Literal,
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
pub enum RoutineKind {
    Test,
    Proc,
    Func,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawMembRoutineKind {
    Proc,
    Func,
}
