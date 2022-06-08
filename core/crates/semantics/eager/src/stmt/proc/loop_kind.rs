use std::sync::Arc;

use text::RangedCustomIdentifier;
use vm::{BoundaryKind, LoopStep, VMLoopKind, VMStackIdx};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopVariant {
    For {
        frame_var: RangedCustomIdentifier,
        initial_boundary: Boundary,
        final_boundary: Boundary,
        step: LoopStep,
    },
    ForExt {
        frame_var: RangedCustomIdentifier,
        final_boundary: Boundary,
        step: LoopStep,
    },
    While {
        condition: Arc<EagerExpr>,
    },
    DoWhile {
        condition: Arc<EagerExpr>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Boundary {
    pub opt_bound: Option<Arc<EagerExpr>>,
    pub kind: BoundaryKind,
}
