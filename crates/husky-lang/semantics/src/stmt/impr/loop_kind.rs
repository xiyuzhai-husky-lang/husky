use vm::{BoundaryKind, LoopStep, VMLoopKind};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopKind {
    For {
        frame_var: CustomIdentifier,
        initial_boundary: Boundary,
        final_boundary: Boundary,
        step: LoopStep,
    },
    ForExt,
    While,
    DoWhile,
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
            LoopKind::ForExt => todo!(),
            LoopKind::While => todo!(),
            LoopKind::DoWhile => todo!(),
        }
    }
}
