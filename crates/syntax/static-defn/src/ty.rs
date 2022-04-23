use dev_utils::{DevSource, StaticDevSource};
use entity_kind::{MemberKind, TyKind};
use visual_syntax::StaticVisualizer;
use vm::*;

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
//         input_placeholders: &'static [StaticInputPlaceholder],
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
            subscopes: &[],
            variant: EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty: $ty },
        }
    };
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MethodStaticDefnKind {
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LinkageSource {
    MemberAccess {
        ref_access: Linkage,
        move_access: Linkage,
        borrow_mut_access: Linkage,
    },
    PureOutput(Linkage),
}
