mod entity;
mod ty;

pub use entity::*;
pub use ty::*;

use entity_syntax::{EntityKind, TyKind};
use visual_syntax::StaticVisualizer;
use vm::{InputContract, RoutineLinkage};

#[derive(Debug, PartialEq, Eq)]
pub struct StaticTraitDecl {
    pub generic_placeholders: &'static [StaticGenericPlaceholder],
    pub methods: &'static [StaticMethodDecl],
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticMethodDecl {
    pub name: &'static str,
    pub this_contract: InputContract,
    pub inputs: &'static [StaticInputDecl],
    pub output_ty: &'static str,
    pub generic_placeholders: &'static [StaticGenericPlaceholder],
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticInputDecl {
    pub contract: InputContract,
    pub ty: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticGenericPlaceholder {
    pub name: &'static str,
    pub variant: StaticGenericPlaceholderVariant,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticGenericPlaceholderVariant {
    Const,
    Type { traits: &'static [&'static str] },
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticFuncDecl {
    pub generic_placeholders: &'static [StaticGenericPlaceholder],
    pub inputs: Vec<StaticInputSignature>,
    pub output: &'static str,
    pub compiled: RoutineLinkage,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticInputSignature {
    pub contract: InputContract,
    pub ty: &'static str,
}
