mod env;
mod opr;

use std::sync::Arc;

pub use env::Env;
pub use opr::*;

use scope::{InputPlaceholder, RangedScope, ScopeKind, ScopePtr};
use vm::Contract;
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
        this: Contract,
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
pub struct RoutineHead {
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
    pub contract: Contract,
    pub scope: ScopePtr,
}

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub struct InputType {
//     pub contract: Contract,
//     pub ty: RangedScope,
// }

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BuiltinScopeData {
    scope_kind: ScopeKind,
    subscopes: [(String, &'static BuiltinScopeData)],
}
