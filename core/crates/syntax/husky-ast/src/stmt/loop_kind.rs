use husky_loop_syntax::{BoundaryKind, LoopStep};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RawLoopKind {
    For {
        frame_var: RangedCustomIdentifier,
        initial_boundary: RawBoundary,
        final_boundary: RawBoundary,
        step: LoopStep,
    },
    ForExt {
        frame_var: RangedCustomIdentifier,
        final_boundary: RawBoundary,
        step: LoopStep,
    },
    While {
        condition: RawExprIdx,
    },
    DoWhile {
        condition: RawExprIdx,
    },
}

impl RawLoopKind {
    pub fn for_loop(
        initial_bound: RawExprIdx,
        initial_comparison: PureBinaryOpr,
        frame_var: RangedCustomIdentifier,
        final_comparison: PureBinaryOpr,
        final_bound: RawExprIdx,
    ) -> AstResult<Self> {
        let (initial_boundary_kind, step) = match initial_comparison {
            PureBinaryOpr::Geq => (BoundaryKind::UpperClosed, LoopStep(-1)),
            PureBinaryOpr::Greater => (BoundaryKind::UpperOpen, LoopStep(-1)),
            PureBinaryOpr::Leq => (BoundaryKind::LowerClosed, LoopStep(1)),
            PureBinaryOpr::Less => (BoundaryKind::LowerOpen, LoopStep(1)),
            _ => todo!(),
        };
        let final_boundary_kind = match final_comparison {
            // ... $frame_var >= $final_bound
            PureBinaryOpr::Geq => BoundaryKind::LowerClosed,
            // ... $frame_var > $final_bound
            PureBinaryOpr::Greater => BoundaryKind::LowerOpen,
            // ... $frame_var <= $final_bound
            PureBinaryOpr::Leq => BoundaryKind::UpperClosed,
            // ... $frame_var < $final_bound
            PureBinaryOpr::Less => BoundaryKind::UpperOpen,
            _ => todo!(),
        };
        check_compatible(initial_boundary_kind, final_boundary_kind)?;
        Ok(Self::For {
            frame_var,
            initial_boundary: RawBoundary {
                opt_bound: Some(initial_bound),
                kind: initial_boundary_kind,
            },
            final_boundary: RawBoundary {
                opt_bound: Some(final_bound),
                kind: final_boundary_kind,
            },
            step,
        })
    }

    pub fn for_loop_with_default_initial(
        frame_var: RangedCustomIdentifier,
        comparison: PureBinaryOpr,
        final_bound: RawExprIdx,
        range: TextRange,
    ) -> AstResult<Self> {
        let final_boundary_kind = match comparison {
            // ill-formed: $frame_var >= $final_bound
            PureBinaryOpr::Geq => err!("invalid form", range)?,
            // ill-formed: $frame_var > $final_bound
            PureBinaryOpr::Greater => err!("invalid form", range)?,
            // well-formed: $frame_var <= $final_bound
            PureBinaryOpr::Leq => BoundaryKind::UpperClosed,
            // well-formed: $frame_var < $final_bound
            PureBinaryOpr::Less => BoundaryKind::UpperOpen,
            _ => todo!(),
        };
        Ok(Self::For {
            frame_var,
            initial_boundary: Default::default(),
            final_boundary: RawBoundary {
                opt_bound: Some(final_bound),
                kind: final_boundary_kind,
            },
            step: LoopStep(1),
        })
    }

    pub fn for_loop_with_default_final(
        initial_bound: RawExprIdx,
        comparison: PureBinaryOpr,
        frame_var: RangedCustomIdentifier,
        range: TextRange,
    ) -> AstResult<Self> {
        let initial_boundary_kind = match comparison {
            // well-formed: $initial_bound >= $frame_var
            PureBinaryOpr::Geq => BoundaryKind::LowerClosed,
            // well-formed: $initial_bound > $frame_var
            PureBinaryOpr::Greater => BoundaryKind::LowerOpen,
            // ill-formed: $initial_bound <= $frame_var
            PureBinaryOpr::Leq => err!("invalid form", range)?,
            // ill-formed: $initial_bound < $frame_var
            PureBinaryOpr::Less => err!("invalid form", range)?,
            _ => return err!(format!("expect comparison"), range),
        };
        Ok(Self::For {
            frame_var,
            initial_boundary: RawBoundary {
                opt_bound: Some(initial_bound),
                kind: initial_boundary_kind,
            },
            final_boundary: Default::default(),
            step: LoopStep(-1),
        })
    }

    pub fn forext_loop(
        frame_var: RangedCustomIdentifier,
        comparison: PureBinaryOpr,
        bound: RawExprIdx,
        range: TextRange,
    ) -> AstResult<Self> {
        let (boundary_kind, step) = match comparison {
            // ... $frame_var >= $final_bound
            PureBinaryOpr::Geq => (BoundaryKind::LowerClosed, LoopStep(-1)),
            // ... $frame_var > $final_bound
            PureBinaryOpr::Greater => (BoundaryKind::LowerOpen, LoopStep(-1)),
            // ... $frame_var <= $final_bound
            PureBinaryOpr::Leq => (BoundaryKind::UpperClosed, LoopStep(1)),
            // ... $frame_var < $final_bound
            PureBinaryOpr::Less => (BoundaryKind::UpperOpen, LoopStep(1)),
            _ => return err!(format!("expect comparison"), range),
        };
        Ok(Self::ForExt {
            frame_var,
            final_boundary: RawBoundary {
                opt_bound: Some(bound),
                kind: boundary_kind,
            },
            step,
        })
    }

    pub fn while_loop(condition: RawExprIdx) -> Self {
        Self::While { condition }
    }

    pub fn do_while_loop(condition: RawExprIdx) -> Self {
        Self::DoWhile { condition }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RawBoundary {
    pub opt_bound: Option<RawExprIdx>,
    pub kind: BoundaryKind,
}

impl Default for RawBoundary {
    fn default() -> Self {
        Self {
            opt_bound: None,
            kind: BoundaryKind::LowerClosed,
        }
    }
}

fn check_compatible(
    initial_boundary_kind: BoundaryKind,
    final_boundary_kind: BoundaryKind,
) -> AstResult<()> {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Direction {
        Incremental,
        Decremental,
    }

    let initial_direction = match initial_boundary_kind {
        BoundaryKind::UpperOpen | BoundaryKind::UpperClosed => Direction::Decremental,
        BoundaryKind::LowerOpen | BoundaryKind::LowerClosed => Direction::Incremental,
    };
    let final_direction = match final_boundary_kind {
        BoundaryKind::UpperOpen | BoundaryKind::UpperClosed => Direction::Incremental,
        BoundaryKind::LowerOpen | BoundaryKind::LowerClosed => Direction::Decremental,
    };
    if initial_direction == final_direction {
        Ok(())
    } else {
        todo!()
    }
}
