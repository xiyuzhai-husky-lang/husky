mod fp;
mod generic_transfer;
mod member;
mod model;
mod specific_routine;

pub use fp::*;
pub use generic_transfer::*;
pub use member::*;
pub use model::*;
pub use specific_routine::*;

use crate::*;
use check_utils::should;
use dev_utils::{DevSource, __StaticDevSource};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum __Linkage {
    Member(&'static __MemberLinkage),
    SpecificTransfer(__SpecificRoutineLinkage),
    GenericTransfer(GenericRoutineLinkage),
    Model(&'static ModelLinkage),
}

impl __Linkage {
    pub fn requires_lazy(&self) -> bool {
        match self {
            __Linkage::Model(_) => true,
            __Linkage::Member { .. } => false,
            __Linkage::SpecificTransfer(_) => false,
            __Linkage::GenericTransfer(_) => false,
        }
    }

    pub fn bind(&self, binding: Binding) -> __SpecificRoutineLinkage {
        match self {
            __Linkage::Member(__Linkage) => __Linkage.bind(binding),
            __Linkage::SpecificTransfer(__Linkage) => *__Linkage,
            __Linkage::Model(_) => todo!(),
            __Linkage::GenericTransfer(_) => todo!(),
        }
    }

    pub fn opt_bind(&self, opt_binding: Option<Binding>) -> __SpecificRoutineLinkage {
        match opt_binding {
            Some(binding) => self.bind(binding),
            None => match self {
                __Linkage::Member { .. } => panic!(),
                __Linkage::SpecificTransfer(__Linkage) => *__Linkage,
                __Linkage::Model(_) => todo!(),
                __Linkage::GenericTransfer(_) => todo!(),
            },
        }
    }
}

impl const From<__SpecificRoutineLinkage> for __Linkage {
    fn from(__Linkage: __SpecificRoutineLinkage) -> Self {
        __Linkage::SpecificTransfer(__Linkage)
    }
}

impl const From<GenericRoutineLinkage> for __Linkage {
    fn from(__Linkage: GenericRoutineLinkage) -> Self {
        __Linkage::GenericTransfer(__Linkage)
    }
}
