mod fp;
mod member;
mod model;
mod transfer;

pub use fp::*;
pub use member::*;
pub use model::*;


use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum __LinkageGroup {
    Transfer(__ResolvedLinkage),
    Member(&'static __MemberLinkageGroup),
    Model(__ModelLinkageGroup),
}

impl __LinkageGroup {
    pub fn requires_lazy(&self) -> bool {
        match self {
            __LinkageGroup::Transfer(_) => false,
            __LinkageGroup::Member(_) => false,
            __LinkageGroup::Model(_) => true,
        }
    }

    #[cfg(feature = "binding")]
    pub fn bind(self, binding: husky_vm_binding::Binding) -> __ResolvedLinkage {
        match self {
            __LinkageGroup::Member(linkage) => linkage.bind(binding),
            __LinkageGroup::Transfer(fp) => fp,
            __LinkageGroup::Model(_) => todo!(),
        }
    }

    pub fn transfer(self) -> __ResolvedLinkage {
        match self {
            __LinkageGroup::Transfer(fp) => fp,
            __LinkageGroup::Member(_) => todo!(),
            __LinkageGroup::Model(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum __StaticLinkageKey {
    VecConstructor {
        element_ty: &'static str,
    },
    TypeCall {
        ty: &'static str,
    },
    Routine {
        route: &'static str,
    },
    Index {
        opd_tys: &'static [&'static str],
    },
    StructField {
        this_ty: &'static str,
        field_ident: &'static str,
    },
    FeatureEagerBlock {
        route: &'static str,
    },
}
