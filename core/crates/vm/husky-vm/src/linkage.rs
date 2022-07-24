mod feature_eager_block;
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
use husky_check_utils::should;
use husky_dev_utils::{DevSource, __StaticDevSource};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LinkageDeprecated {
    Member(&'static __MemberLinkage),
    SpecificTransfer(__SpecificRoutineLinkage),
    GenericTransfer(GenericRoutineLinkage),
    Model(ModelLinkage),
}

impl LinkageDeprecated {
    pub const fn specific(self) -> __SpecificRoutineLinkage {
        match self {
            LinkageDeprecated::SpecificTransfer(linkage) => linkage,
            LinkageDeprecated::Member(_) => panic!(),
            LinkageDeprecated::GenericTransfer(_) => panic!(),
            LinkageDeprecated::Model(_) => panic!(),
        }
    }

    pub const fn requires_lazy(&self) -> bool {
        match self {
            LinkageDeprecated::Model(_) => true,
            LinkageDeprecated::Member { .. } => false,
            LinkageDeprecated::SpecificTransfer(_) => false,
            LinkageDeprecated::GenericTransfer(_) => false,
        }
    }

    pub fn bind(&self, binding: Binding) -> __SpecificRoutineLinkage {
        match self {
            LinkageDeprecated::Member(__Linkage) => __Linkage.bind(binding),
            LinkageDeprecated::SpecificTransfer(__Linkage) => *__Linkage,
            LinkageDeprecated::Model(_) => todo!(),
            LinkageDeprecated::GenericTransfer(_) => todo!(),
        }
    }

    pub fn opt_bind(&self, opt_binding: Option<Binding>) -> __SpecificRoutineLinkage {
        match opt_binding {
            Some(binding) => self.bind(binding),
            None => match self {
                LinkageDeprecated::Member { .. } => panic!(),
                LinkageDeprecated::SpecificTransfer(__Linkage) => *__Linkage,
                LinkageDeprecated::Model(_) => todo!(),
                LinkageDeprecated::GenericTransfer(_) => todo!(),
            },
        }
    }
}

impl const From<__SpecificRoutineLinkage> for LinkageDeprecated {
    fn from(__Linkage: __SpecificRoutineLinkage) -> Self {
        LinkageDeprecated::SpecificTransfer(__Linkage)
    }
}

impl const From<GenericRoutineLinkage> for LinkageDeprecated {
    fn from(__Linkage: GenericRoutineLinkage) -> Self {
        LinkageDeprecated::GenericTransfer(__Linkage)
    }
}
