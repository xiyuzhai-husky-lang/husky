use husky_word::CustomIdentifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VMLoopKind {
    For {
        frame_var: CustomIdentifier,
        initial_boundary_kind: BoundaryKind,
        final_boundary_kind: BoundaryKind,
        step: LoopStep,
    },
    ForExt {
        frame_var: CustomIdentifier,
        frame_varidx: VMStackIdx,
        final_boundary_kind: BoundaryKind,
        step: LoopStep,
    },
    Loop,
}
