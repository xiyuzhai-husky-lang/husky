mod env;
mod opr;

use std::sync::Arc;

pub use env::Env;
pub use opr::*;

use scope::{RangedScope, ScopeKind, ScopePtr};
use vm::{InputContract, PrimitiveValue};
use word::CustomIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TyKind {
    Enum(Vec<CustomIdentifier>),
    Struct,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MembKind {
    MembVar {
        ty: MembType,
    },
    MembFunc {
        this: InputContract,
        inputs: Vec<InputPlaceholder>,
        output: RangedScope,
        args: Vec<(CustomIdentifier, GenericPlaceholderKind)>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RoutineKind {
    Test,
    Proc,
    Func,
    Def,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutineDecl {
    pub funcname: CustomIdentifier,
    pub generics: Vec<GenericPlaceholder>,
    pub input_placeholders: Arc<Vec<InputPlaceholder>>,
    pub output: RangedScope,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GenericPlaceholderKind {
    Const,
    Type { traits: Vec<RangedScope> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericPlaceholder {
    pub ident: CustomIdentifier,
    pub kind: GenericPlaceholderKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MembType {
    pub contract: InputContract,
    pub scope: ScopePtr,
}

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub struct InputType {
//     pub contract: InputContract,
//     pub ty: RangedScope,
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputPlaceholder {
    pub ident: CustomIdentifier,
    pub contract: InputContract,
    pub ty: RangedScope,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InitKind {
    Let,
    Var,
    Decl,
}

impl std::fmt::Display for InitKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            InitKind::Let => "let",
            InitKind::Var => "var",
            InitKind::Decl => "decl",
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BuiltinScopeData {
    scope_kind: ScopeKind,
    subscopes: [(String, &'static BuiltinScopeData)],
}
