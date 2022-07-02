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
use dev_utils::{DevSource, StaticDevSource};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Linkage {
    Member(&'static MemberLinkage),
    SpecificTransfer(SpecificRoutineLinkage),
    GenericTransfer(GenericRoutineLinkage),
    Model(&'static ModelLinkage),
}

impl Linkage {
    pub fn requires_lazy(&self) -> bool {
        match self {
            Linkage::Model(_) => true,
            Linkage::Member { .. } => false,
            Linkage::SpecificTransfer(_) => false,
            Linkage::GenericTransfer(_) => false,
        }
    }

    pub fn bind(&self, binding: Binding) -> SpecificRoutineLinkage {
        match self {
            Linkage::Member(linkage) => linkage.bind(binding),
            Linkage::SpecificTransfer(linkage) => *linkage,
            Linkage::Model(_) => todo!(),
            Linkage::GenericTransfer(_) => todo!(),
        }
    }

    pub fn opt_bind(&self, opt_binding: Option<Binding>) -> SpecificRoutineLinkage {
        match opt_binding {
            Some(binding) => self.bind(binding),
            None => match self {
                Linkage::Member { .. } => panic!(),
                Linkage::SpecificTransfer(linkage) => *linkage,
                Linkage::Model(_) => todo!(),
                Linkage::GenericTransfer(_) => todo!(),
            },
        }
    }
}

impl const From<SpecificRoutineLinkage> for Linkage {
    fn from(linkage: SpecificRoutineLinkage) -> Self {
        Linkage::SpecificTransfer(linkage)
    }
}

impl const From<GenericRoutineLinkage> for Linkage {
    fn from(linkage: GenericRoutineLinkage) -> Self {
        Linkage::GenericTransfer(linkage)
    }
}
