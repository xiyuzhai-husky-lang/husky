mod generic_transfer;
mod model;
mod specific_routine;

pub use generic_transfer::*;
pub use model::*;
pub use specific_routine::*;

use crate::*;
use check_utils::should;
use dev_utils::{DevSource, StaticDevSource};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Linkage {
    MemberAccess {
        copy_access: SpecificRoutineLinkage,
        eval_ref_access: SpecificRoutineLinkage,
        temp_ref_access: SpecificRoutineLinkage,
        temp_mut_access: SpecificRoutineLinkage,
        move_access: SpecificRoutineLinkage,
    },
    SpecificTransfer(SpecificRoutineLinkage),
    GenericTransfer(GenericRoutineLinkage),
    Model(&'static ModelLinkage),
}

impl Linkage {
    pub fn requires_lazy(&self) -> bool {
        match self {
            Linkage::Model(_) => true,
            Linkage::MemberAccess { .. } => false,
            Linkage::SpecificTransfer(_) => false,
            Linkage::GenericTransfer(_) => false,
        }
    }

    pub fn bind(&self, binding: Binding) -> SpecificRoutineLinkage {
        match self {
            Linkage::MemberAccess {
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
            Linkage::SpecificTransfer(linkage) => *linkage,
            Linkage::Model(_) => todo!(),
            Linkage::GenericTransfer(_) => todo!(),
        }
    }

    pub fn opt_bind(&self, opt_binding: Option<Binding>) -> SpecificRoutineLinkage {
        match opt_binding {
            Some(binding) => self.bind(binding),
            None => match self {
                Linkage::MemberAccess { .. } => panic!(),
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
