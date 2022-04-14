mod entity;
mod ty;

pub use entity::*;
pub use ty::*;

use entity_syntax::{EntityKind, TyKind};
use visual_syntax::StaticVisualizer;
use vm::{InputContract, RoutineLinkage};

#[derive(Debug, PartialEq, Eq)]
pub struct StaticTraitDecl {
    pub route: &'static str,
    pub generic_placeholders: &'static [StaticGenericPlaceholder],
    pub members: &'static [StaticTraitMemberDecl],
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticMethodDecl {
    pub name: &'static str,
    pub this_contract: InputContract,
    pub inputs: &'static [StaticInputDecl],
    pub output_ty: &'static str,
    pub generic_placeholders: &'static [StaticGenericPlaceholder],
    pub kind: StaticMethodKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticMethodKind {
    Type,
    Trait(&'static str),
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticInputDecl {
    pub contract: InputContract,
    pub ty: &'static str,
    pub name: &'static str,
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
    pub inputs: Vec<StaticInputDecl>,
    pub output: &'static str,
    pub compiled: RoutineLinkage,
}

pub static CLONE_TRAIT_DECL: StaticTraitDecl = StaticTraitDecl {
    route: "Clone",
    members: &[StaticTraitMemberDecl::Method(StaticMethodDecl {
        name: "clone",
        this_contract: vm::InputContract::Pure,
        inputs: &[],
        output_ty: "This",
        generic_placeholders: &[],
        kind: StaticMethodKind::Trait("Clone"),
    })],
    generic_placeholders: &[],
};

pub static STATIC_VEC_DECL: StaticTyDecl = StaticTyDecl {
    base_ty: "Vec",
    generic_placeholders: &[StaticGenericPlaceholder {
        name: "E",
        variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
    }],
    trait_impls: &[StaticTraitImplDecl { route: "Clone" }],
    fields: &[],
    type_members: &[
        StaticTypeMemberDecl::Method(StaticMethodDecl {
            name: "len",
            this_contract: InputContract::Pure,
            inputs: &[],
            output_ty: "i32",
            generic_placeholders: &[],
            kind: StaticMethodKind::Type,
        }),
        StaticTypeMemberDecl::Method(StaticMethodDecl {
            name: "push",
            this_contract: InputContract::BorrowMut,
            inputs: &[StaticInputDecl {
                contract: InputContract::Move,
                ty: "E",
                name: "element",
            }],
            output_ty: "void",
            generic_placeholders: &[],
            kind: StaticMethodKind::Type,
        }),
        StaticTypeMemberDecl::Method(StaticMethodDecl {
            name: "pop",
            this_contract: InputContract::BorrowMut,
            inputs: &[],
            output_ty: "E",
            generic_placeholders: &[],
            kind: StaticMethodKind::Type,
        }),
    ],
    variants: &[],
    kind: TyKind::Vec,
};
