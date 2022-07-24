mod generic_transfer;

pub use generic_transfer::*;

use crate::*;
use husky_check_utils::should;
use husky_dev_utils::{DevSource, __StaticDevSource};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum __VirtualLinkage {
    Member(&'static __MemberLinkage),
    SpecificTransfer(__LinkageFp),
    GenericTransfer(GenericLinkageFp),
    Model(__ModelLinkage),
}

impl __VirtualLinkage {
    pub const fn specific(self) -> __LinkageFp {
        match self {
            __VirtualLinkage::SpecificTransfer(linkage) => linkage,
            __VirtualLinkage::Member(_) => panic!(),
            __VirtualLinkage::GenericTransfer(_) => panic!(),
            __VirtualLinkage::Model(_) => panic!(),
        }
    }

    pub const fn requires_lazy(&self) -> bool {
        match self {
            __VirtualLinkage::Model(_) => true,
            __VirtualLinkage::Member { .. } => false,
            __VirtualLinkage::SpecificTransfer(_) => false,
            __VirtualLinkage::GenericTransfer(_) => false,
        }
    }

    pub fn bind(&self, binding: Binding) -> __LinkageFp {
        match self {
            __VirtualLinkage::Member(__Linkage) => todo!(),
            // __Linkage.bind(binding),
            __VirtualLinkage::SpecificTransfer(__Linkage) => *__Linkage,
            __VirtualLinkage::Model(_) => todo!(),
            __VirtualLinkage::GenericTransfer(_) => todo!(),
        }
    }

    pub fn opt_bind(&self, opt_binding: Option<Binding>) -> __LinkageFp {
        match opt_binding {
            Some(binding) => self.bind(binding),
            None => match self {
                __VirtualLinkage::Member { .. } => panic!(),
                __VirtualLinkage::SpecificTransfer(__Linkage) => *__Linkage,
                __VirtualLinkage::Model(_) => todo!(),
                __VirtualLinkage::GenericTransfer(_) => todo!(),
            },
        }
    }
}

impl const From<__LinkageFp> for __VirtualLinkage {
    fn from(__Linkage: __LinkageFp) -> Self {
        __VirtualLinkage::SpecificTransfer(__Linkage)
    }
}

impl const From<GenericLinkageFp> for __VirtualLinkage {
    fn from(__Linkage: GenericLinkageFp) -> Self {
        __VirtualLinkage::GenericTransfer(__Linkage)
    }
}
