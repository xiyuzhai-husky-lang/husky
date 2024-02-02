use husky_coword::Ident;
use husky_eth_term::EthTerm;

use crate::*;

// ad hoc
type ModuleRange = ();

#[derive(Debug)]
pub struct MutationData {
    pub range: ModuleRange,
    pub kind: MutationDataVariant,
    pub ty: EthTerm,
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
