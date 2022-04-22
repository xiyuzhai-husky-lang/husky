use dev_utils::{DevSource, StaticDevSource};
use entity_kind::{MemberKind, TyKind};
use visual_syntax::StaticVisualizer;
use vm::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct StaticFieldDefn {
    pub name: &'static str,
    pub variant: StaticFieldVariant,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticTraitImplDefn {
    pub route: &'static str,
    pub member_impls: &'static [TraitMemberImplStaticDefn],
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
pub enum TypeMemberStaticDefn {
    Field,
    Method(StaticMethodDefn),
    Call,
}

impl TypeMemberStaticDefn {
    pub fn name(&self) -> &'static str {
        match self {
            TypeMemberStaticDefn::Field => todo!(),
            TypeMemberStaticDefn::Method(method) => method.name,
            TypeMemberStaticDefn::Call => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TraitMemberStaticDefn {
    Method(StaticMethodDefn),
    Call,
    Type {
        name: &'static str,
        traits: &'static [&'static str],
    },
}

impl TraitMemberStaticDefn {
    pub fn name(&self) -> &'static str {
        match self {
            TraitMemberStaticDefn::Method(method_decl) => method_decl.name,
            TraitMemberStaticDefn::Call => todo!(),
            TraitMemberStaticDefn::Type { name, traits } => name,
        }
    }

    pub fn kind(&self) -> MemberKind {
        match self {
            TraitMemberStaticDefn::Method(_) => MemberKind::Method,
            TraitMemberStaticDefn::Call => MemberKind::Call,
            TraitMemberStaticDefn::Type { .. } => MemberKind::AssociatedType,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitMemberImplStaticDefn {
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
        TraitMemberImplStaticDefn {
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
