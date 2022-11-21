use husky_identifier::Identifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VMLoopKind {
    For {
        frame_var: Identifier,
        initial_boundary_kind: BoundaryKind,
        final_boundary_kind: BoundaryKind,
        step: LoopStep,
    },
    ForExt {
        frame_var: Identifier,
        frame_varidx: VMStackIdx,
        final_boundary_kind: BoundaryKind,
        step: LoopStep,
    },
    Loop,
}
