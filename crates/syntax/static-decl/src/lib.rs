mod entity;
mod ty;

pub use entity::*;
pub use ty::*;

use entity_syntax::{EntityKind, TyKind};
use visual_syntax::StaticVisualizer;
use vm::{InputContract, RoutineLinkage};
use word::CustomIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StaticMethodDecl {
    pub name: &'static str,
    pub this_contract: InputContract,
    pub inputs: &'static [StaticInputDecl],
    pub output_ty: &'static str,
    pub generic_placeholders: &'static [StaticGenericPlaceholder],
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StaticInputDecl {
    pub contract: InputContract,
    pub ty: &'static str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StaticGenericPlaceholder {
    pub name: &'static str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StaticFuncDecl {
    pub inputs: Vec<StaticInputSignature>,
    pub output: &'static str,
    pub compiled: RoutineLinkage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticInputSignature {
    pub contract: InputContract,
    pub ty: &'static str,
}
