use dev_utils::{DevSource, StaticDevSource};
use entity_kind::{MemberKind, TyKind};
use visual_syntax::StaticVisualizer;
use vm::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct StaticTypeDefn {
    pub base_route: &'static str,
    pub generic_placeholders: &'static [StaticGenericPlaceholder],
    pub trait_impls: &'static [StaticTraitImplDefn],
    pub type_members: &'static [StaticTypeMemberDefn],
    pub variants: &'static [StaticEnumVariantDecl],
    pub kind: TyKind,
    pub visualizer: StaticVisualizer,
    pub opt_type_call: Option<&'static StaticCallDefn>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticFieldDefn {
    pub name: &'static str,
    pub variant: StaticFieldVariant,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticTraitImplDefn {
    pub route: &'static str,
    pub member_impls: &'static [StaticTraitMemberImplDefn],
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticGenericArgument {
    EntityRoute(&'static str),
    ConstUsize(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticFieldVariant {}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticEnumVariantDecl {}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticTypeMemberDefn {
    Field,
    Method(StaticMethodDefn),
    Call,
}

impl StaticTypeMemberDefn {
    pub fn name(&self) -> &'static str {
        match self {
            StaticTypeMemberDefn::Field => todo!(),
            StaticTypeMemberDefn::Method(method) => method.name,
            StaticTypeMemberDefn::Call => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticTraitMemberDecl {
    Method(StaticMethodDefn),
    Call,
    Type {
        name: &'static str,
        traits: &'static [&'static str],
    },
}

impl StaticTraitMemberDecl {
    pub fn name(&self) -> &'static str {
        match self {
            StaticTraitMemberDecl::Method(method_decl) => method_decl.name,
            StaticTraitMemberDecl::Call => todo!(),
            StaticTraitMemberDecl::Type { name, traits } => name,
        }
    }

    pub fn kind(&self) -> MemberKind {
        match self {
            StaticTraitMemberDecl::Method(_) => MemberKind::Method,
            StaticTraitMemberDecl::Call => MemberKind::Call,
            StaticTraitMemberDecl::Type { .. } => MemberKind::AssociatedType,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticTraitMemberImplDefn {
    pub dev_src: StaticDevSource,
    pub name: &'static str,
    pub variant: StaticTraitMemberImplDefnVariant,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticTraitMemberImplDefnVariant {
    Type {
        route: &'static str,
    },
    Method {
        this_contract: InputContract,
        input_placeholders: &'static [StaticInputPlaceholder],
        output: &'static str,
        ref_access: Linkage,
        move_access: Linkage,
        borrow_mut_access: Linkage,
    },
}

#[macro_export]
macro_rules! associated_type {
    ($name: expr, $route: expr) => {
        StaticTraitMemberImplDefn {
            dev_src: dev_utils::static_dev_src!(),
            name: $name,
            variant: StaticTraitMemberImplDefnVariant::Type { route: $route },
        }
    };
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticMethodDefn {
    pub name: &'static str,
    pub this_contract: InputContract,
    pub inputs: &'static [StaticInputPlaceholder],
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
