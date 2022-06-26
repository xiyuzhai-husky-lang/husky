use std::sync::atomic::AtomicUsize;

use entity_route::EntityRoutePtr;
use file::FilePtr;
use text::TextRange;
use word::Identifier;

use crate::*;

#[derive(Debug)]
pub struct MutationData<'eval> {
    pub file: FilePtr,
    pub range: TextRange,
    pub kind: MutationDataVariant,
    pub ty: EntityRoutePtr,
    pub before: Option<EvalValue<'eval>>,
    pub after: EvalValue<'eval>,
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
            MutationDataVariant::Block { stack_idx, varname } => stack_idx,
        }
    }
}
