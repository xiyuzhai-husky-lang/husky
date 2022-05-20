use entity_route::EntityRoutePtr;
use file::FilePtr;
use text::TextRange;
use word::Identifier;

use crate::*;

#[derive(Debug, Clone)]
pub struct MutationData<'eval> {
    pub file: FilePtr,
    pub kind: MutationDataKind,
    pub ty: EntityRoutePtr,
    pub before: Option<EvalValue<'eval>>,
    pub after: EvalValue<'eval>,
}

#[derive(Debug, Clone)]
pub enum MutationDataKind {
    Exec {
        range: TextRange,
    },
    Block {
        stack_idx: StackIdx,
        varname: Identifier,
    },
}

impl<'eval> MutationData<'eval> {
    pub fn varidx(&self) -> StackIdx {
        match self.kind {
            MutationDataKind::Exec { range } => panic!(),
            MutationDataKind::Block { stack_idx, varname } => stack_idx,
        }
    }
}
