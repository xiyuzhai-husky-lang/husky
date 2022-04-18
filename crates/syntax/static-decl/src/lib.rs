mod entity;
mod ty;

pub use entity::*;
pub use ty::*;

use entity_kind::{EntityKind, TyKind};
use visual_syntax::{StaticVisualizer, TRIVIAL_VISUALIZER};
use vm::{BoxedValue, InputContract, Linkage, OutputContract, StackValue, VirtualTy};

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
    pub output_contract: OutputContract,
    pub generic_placeholders: &'static [StaticGenericPlaceholder],
    pub kind: StaticMethodKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticMethodKind {
    TypeMethod,
    TraitMethod(&'static str),
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
pub struct StaticCallDecl {
    pub generic_placeholders: &'static [StaticGenericPlaceholder],
    pub inputs: Vec<StaticInputDecl>,
    pub output_ty: &'static str,
    pub output_contract: OutputContract,
    pub linkage: Linkage,
}

pub static CLONE_TRAIT_DECL: StaticTraitDecl = StaticTraitDecl {
    route: "Clone",
    members: &[StaticTraitMemberDecl::Method(StaticMethodDecl {
        name: "clone",
        this_contract: vm::InputContract::Pure,
        inputs: &[],
        output_ty: "This",
        generic_placeholders: &[],
        kind: StaticMethodKind::TraitMethod("Clone"),
        output_contract: OutputContract::Pure,
    })],
    generic_placeholders: &[],
};

pub static VEC_TYPE_DECL: StaticTypeDecl = StaticTypeDecl {
    base_route: "Vec",
    generic_placeholders: &[StaticGenericPlaceholder {
        name: "E",
        variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
    }],
    trait_impls: &[StaticTraitImplDecl { route: "Clone" }],
    type_members: &[
        StaticTypeMemberDecl::Method(StaticMethodDecl {
            name: "len",
            this_contract: InputContract::Pure,
            inputs: &[],
            output_ty: "i32",
            generic_placeholders: &[],
            kind: StaticMethodKind::TypeMethod,
            output_contract: OutputContract::Pure,
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
            kind: StaticMethodKind::TypeMethod,
            output_contract: OutputContract::Pure,
        }),
        StaticTypeMemberDecl::Method(StaticMethodDecl {
            name: "pop",
            this_contract: InputContract::BorrowMut,
            inputs: &[],
            output_ty: "E",
            generic_placeholders: &[],
            kind: StaticMethodKind::TypeMethod,
            output_contract: OutputContract::Pure,
        }),
    ],
    variants: &[],
    kind: TyKind::Vec,
    visualizer: TRIVIAL_VISUALIZER,
    opt_type_call: Some(&NEW_VEC_DECL),
};

static NEW_VEC_DECL: StaticCallDecl = StaticCallDecl {
    generic_placeholders: &[],
    inputs: vec![],
    output_ty: "Vec<E>",
    output_contract: OutputContract::Pure,
    linkage: Linkage {
        call: |values| Ok(StackValue::Boxed(BoxedValue::new(Vec::<VirtualTy>::new()))),
        nargs: 0,
    },
};
