use std::sync::Arc;

use husky_text::RangedCustomIdentifier;
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

impl Boundary {
    fn needs_context(&self) -> bool {
        if let Some(ref bound) = self.opt_bound {
            bound.needs_context
        } else {
            false
        }
    }
}

impl LoopVariant {
    pub(crate) fn needs_context(&self) -> bool {
        match self {
            LoopVariant::For {
                frame_var,
                initial_boundary,
                final_boundary,
                step,
            } => initial_boundary.needs_context() || final_boundary.needs_context(),
            LoopVariant::ForExt {
                frame_var,
                final_boundary,
                step,
            } => final_boundary.needs_context(),
            LoopVariant::While { condition } => condition.needs_context,
            LoopVariant::DoWhile { condition } => condition.needs_context,
        }
    }
}
