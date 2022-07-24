mod fp;
mod member;
mod model;
mod transfer;

pub use fp::*;
pub use member::*;
pub use model::*;
pub use transfer::*;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum __Linkage {
    Transfer(__LinkageFp),
    Member(&'static __MemberLinkage),
    Model(__ModelLinkage),
}

impl __Linkage {
    pub fn requires_lazy(&self) -> bool {
        match self {
            __Linkage::Transfer(_) => false,
            __Linkage::Member(_) => false,
            __Linkage::Model(_) => true,
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
    StructEagerField {
        this_ty: &'static str,
        field_ident: &'static str,
    },
    FeatureEagerBlock {
        route: &'static str,
    },
}
