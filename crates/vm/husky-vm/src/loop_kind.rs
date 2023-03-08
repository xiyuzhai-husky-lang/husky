use husky_word::Ident;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VMLoopKind {
    For {
        frame_var: Ident,
        initial_boundary_kind: BoundaryKind,
        final_boundary_kind: BoundaryKind,
        step: LoopStep,
    },
    ForExt {
        frame_var: Ident,
        frame_varidx: VMStackIdx,
        final_boundary_kind: BoundaryKind,
        step: LoopStep,
    },
    Loop,
}
