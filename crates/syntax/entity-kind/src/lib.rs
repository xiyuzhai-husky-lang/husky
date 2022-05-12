use serde::{Deserialize, Serialize};
// use token::{Special, Token, TokenKind};
use word::{Keyword, TyKeyword};

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
    Routine,
    Feature,
    Pattern,
    EnumLiteral,
    Main,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemberKind {
    Field,
    Method,
    Call,
    TraitAssociatedType,
    TraitAssociatedConstSize,
    TraitAssociatedAny,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldKind {
    StructOriginal,
    StructDerived,
    RecordOriginal,
    RecordDerived,
}
