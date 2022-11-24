use husky_identifier::Identifier;
use husky_source_path::SourcePath;
use husky_term::Term;
use husky_text::TextRange;

use crate::*;

#[derive(Debug)]
pub struct MutationData<'eval> {
    pub file: SourcePath,
    pub range: TextRange,
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
