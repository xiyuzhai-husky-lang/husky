use husky_coword::Ident;
use husky_ethereal_term::EtherealTerm;
use husky_text::ModuleRange;

use crate::*;

#[derive(Debug)]
pub struct MutationData {
    pub range: ModuleRange,
    pub kind: MutationDataVariant,
    pub ty: EtherealTerm,
    pub before: Option<RegularValue>,
    pub after: RegularValue,
}

#[derive(Debug)]
pub enum MutationDataVariant {
    Exec,
    Block {
        stack_idx: VMStackIdx,
        varname: Ident,
    },
}

impl MutationData {
    pub fn varidx(&self) -> VMStackIdx {
        match self.kind {
            MutationDataVariant::Exec => panic!(),
            MutationDataVariant::Block { stack_idx, .. } => stack_idx,
        }
    }
}
