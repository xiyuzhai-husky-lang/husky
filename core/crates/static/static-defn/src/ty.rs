use check_utils::should;
use dev_utils::{DevSource, StaticDevSource};
use entity_kind::{MemberKind, TyKind};
use visual_syntax::StaticVisualizer;
use vm::{ModelLinkage, *};

use crate::*;

// #[derive(Debug, PartialEq, Eq)]
// pub struct StaticFieldDefn {
//     pub name: &'static str,
//     pub variant: StaticFieldVariant,
// }

#[derive(Debug, PartialEq, Eq)]
pub struct StaticTraitImplDefn {
    pub trai: &'static str,
    pub member_impls: &'static [EntityStaticDefn],
    pub dev_src: StaticDevSource,
}

// #[derive(Debug, PartialEq, Eq)]
// pub enum StaticGenericArgument {
//     EntityRoute(&'static str),
//     ConstUsize(usize),
// }

#[derive(Debug, PartialEq, Eq)]
pub enum StaticFieldVariant {}

// #[derive(Debug, PartialEq, Eq)]
// pub struct StaticEnumVariantDecl {}

// #[derive(Debug, PartialEq, Eq)]
// pub struct EntityStaticDefn {
//     pub dev_src: StaticDevSource,
//     pub name: &'static str,
//     pub variant: StaticTraitMemberImplDefnVariant,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub enum StaticTraitMemberImplDefnVariant {
//     Type {
//         route: &'static str,
//     },
//     Method {
//         this_contract: InputContract,
//         parameters: &'static [StaticInputPlaceholder],
//         output: &'static str,
//         ref_access: Linkage,
//         move_access: Linkage,
//         borrow_mut_access: Linkage,
//     },
// }

#[macro_export]
macro_rules! associated_type_impl {
    ($name: expr, $ty: expr) => {
        EntityStaticDefn {
            dev_src: dev_utils::static_dev_src!(),
            name: $name,
            items: &[],
            variant: EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty: $ty },
        }
    };
}

#[derive(Debug, PartialEq, Eq)]
pub enum MethodStaticDefnVariant {
    TypeMethod {
        source: LinkageSource,
    },
    TraitMethod {
        opt_default_source: Option<LinkageSource>,
    },
    TraitMethodImpl {
        opt_source: Option<LinkageSource>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LinkageSource {
    MemberAccess {
        copy_access: RoutineLinkage,
        eval_ref_access: RoutineLinkage,
        temp_ref_access: RoutineLinkage,
        temp_mut_access: RoutineLinkage,
        move_access: RoutineLinkage,
    },
    Transfer(RoutineLinkage),
    Model(&'static ModelLinkage),
}

impl LinkageSource {
    pub fn bind(&self, binding: Binding) -> RoutineLinkage {
        match self {
            LinkageSource::MemberAccess {
                copy_access,
                eval_ref_access,
                temp_ref_access,
                temp_mut_access,
                move_access,
            } => match binding {
                Binding::EvalRef => *eval_ref_access,
                Binding::TempRef => *temp_ref_access,
                Binding::TempRefMut => *temp_mut_access,
                Binding::Move => *move_access,
                Binding::Copy => *copy_access,
            },
            LinkageSource::Transfer(linkage) => *linkage,
            LinkageSource::Model(_) => todo!(),
        }
    }

    pub fn opt_bind(&self, opt_binding: Option<Binding>) -> RoutineLinkage {
        match opt_binding {
            Some(binding) => self.bind(binding),
            None => match self {
                LinkageSource::MemberAccess { .. } => panic!(),
                LinkageSource::Transfer(linkage) => *linkage,
                LinkageSource::Model(_) => todo!(),
            },
        }
    }
}
