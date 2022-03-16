use vm::{BoundaryKind, LoopStep, StackIdx, VMLoopKind};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopKind {
    For {
        frame_var: CustomIdentifier,
        initial_boundary: Boundary,
        final_boundary: Boundary,
        step: LoopStep,
    },
    ForExt {
        frame_var: CustomIdentifier,
        frame_varidx: StackIdx,
        final_boundary: Boundary,
        step: LoopStep,
    },
    While {
        condition: Arc<Expr>,
    },
    DoWhile {
        condition: Arc<Expr>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Boundary {
    pub opt_bound: Option<Arc<Expr>>,
    pub kind: BoundaryKind,
}

impl Into<VMLoopKind> for &LoopKind {
    fn into(self) -> VMLoopKind {
        match self {
            LoopKind::For {
                frame_var,
                initial_boundary,
                final_boundary,
                step,
            } => VMLoopKind::For {
                initial_boundary_kind: initial_boundary.kind,
                final_boundary_kind: final_boundary.kind,
                step: *step,
                frame_var: *frame_var,
            },
            LoopKind::ForExt {
                frame_var,
                final_boundary,
                frame_varidx,
                step,
            } => VMLoopKind::ForExt {
                final_boundary_kind: final_boundary.kind,
                step: *step,
                frame_var: *frame_var,
                frame_varidx: *frame_varidx,
            },
            LoopKind::While { .. } | LoopKind::DoWhile { .. } => VMLoopKind::Loop,
        }
    }
}
