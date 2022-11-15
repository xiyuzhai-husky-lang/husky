use husky_file::FileItd;
use husky_term::Ty;
use husky_text::TextRange;
use husky_word::Identifier;

use crate::*;

#[derive(Debug)]
pub struct MutationData<'eval> {
    pub file: FileItd,
    pub range: TextRange,
    pub kind: MutationDataVariant,
    pub ty: Ty,
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
