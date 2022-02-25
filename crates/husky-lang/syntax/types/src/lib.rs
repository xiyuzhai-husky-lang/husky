mod env;
mod opr;

use std::sync::Arc;

pub use env::Env;
pub use opr::*;

use scope::{ScopeKind, ScopePtr};
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
        inputs: Vec<InputType>,
        output: InputType,
        args: Vec<(CustomIdentifier, GenericPlaceholderKind)>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FuncKind {
    Test,
    Proc,
    Func,
    Def,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncDecl {
    pub funcname: CustomIdentifier,
    pub generics: Vec<GenericPlaceholder>,
    pub input_contracts: Arc<Vec<(CustomIdentifier, InputType)>>,
    pub output: ScopePtr,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GenericPlaceholderKind {
    Const,
    Type { traits: Vec<ScopePtr> },
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InputType {
    pub contract: InputContract,
    pub ty: ScopePtr,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InitKind {
    Let,
    Var,
    Functional,
}

impl std::fmt::Display for InitKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            InitKind::Let => "let",
            InitKind::Var => "var",
            InitKind::Functional => "functional",
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BuiltinScopeData {
    scope_kind: ScopeKind,
    subscopes: [(String, &'static BuiltinScopeData)],
}
