use husky_term::Term;
use husky_text::ModuleRange;
use husky_word::Ident;

use crate::*;

#[derive(Debug)]
pub struct MutationData<'eval> {
    pub range: ModuleRange,
    pub kind: MutationDataVariant,
    pub ty: Term,
    pub before: Option<__Register<'eval>>,
    pub after: __Register<'eval>,
}

#[derive(Debug)]
pub enum MutationDataVariant {
    Exec,
    Block {
        stack_idx: VMStackIdx,
        varname: Ident,
    },
}

impl<'eval> MutationData<'eval> {
    pub fn varidx(&self) -> VMStackIdx {
        match self.kind {
            MutationDataVariant::Exec => panic!(),
            MutationDataVariant::Block { stack_idx, .. } => stack_idx,
        }
    }
}
