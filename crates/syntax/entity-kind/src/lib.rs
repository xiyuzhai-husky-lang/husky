use serde::{Deserialize, Serialize};
// use token::{Special, Token, TokenKind};
use word::{Keyword, Paradigm, TyKeyword};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityKind {
    Module,
    Type(TyKind),
    Trait,
    Member(MemberKind),
    Function { is_lazy: bool },
    Feature,
    EnumLiteral,
    Main,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnumVariantKind {
    Constant,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemberKind {
    Field,
    Method,
    Call,
    TraitAssociatedType,
    TraitAssociatedConstSize,
    TraitAssociatedAny,
}
