use husky_term::Term;
use husky_text::FileRange;
use husky_word::Identifier;

use crate::*;

#[derive(Debug)]
pub struct MutationData<'eval> {
    pub range: FileRange,
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
        varname: Identifier,
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
