use husky_loop_syntax::{BoundaryKind, LoopStep};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RawLoopKind {
    For {
        frame_var: RangedIdentifier,
        initial_boundary: RawBoundary,
        final_boundary: RawBoundary,
        step: LoopStep,
    },
    ForExt {
        frame_var: RangedIdentifier,
        final_boundary: RawBoundary,
        step: LoopStep,
    },
    While {
        condition: ExprIdx,
    },
    DoWhile {
        condition: ExprIdx,
    },
}

impl RawLoopKind {
    pub fn for_loop(
        initial_bound: ExprIdx,
        initial_comparison: BinaryComparisonOpr,
        frame_var: RangedIdentifier,
        final_comparison: BinaryComparisonOpr,
        final_bound: ExprIdx,
    ) -> AstResult<Self> {
        let (initial_boundary_kind, step) = match initial_comparison {
            BinaryComparisonOpr::Geq => (BoundaryKind::UpperClosed, LoopStep(-1)),
            BinaryComparisonOpr::Greater => (BoundaryKind::UpperOpen, LoopStep(-1)),
            BinaryComparisonOpr::Leq => (BoundaryKind::LowerClosed, LoopStep(1)),
            BinaryComparisonOpr::Less => (BoundaryKind::LowerOpen, LoopStep(1)),
            _ => todo!(),
        };
        let final_boundary_kind = match final_comparison {
            // ... $frame_var >= $final_bound
            BinaryComparisonOpr::Geq => BoundaryKind::LowerClosed,
            // ... $frame_var > $final_bound
            BinaryComparisonOpr::Greater => BoundaryKind::LowerOpen,
            // ... $frame_var <= $final_bound
            BinaryComparisonOpr::Leq => BoundaryKind::UpperClosed,
            // ... $frame_var < $final_bound
            BinaryComparisonOpr::Less => BoundaryKind::UpperOpen,
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
        frame_var: RangedIdentifier,
        comparison: BinaryComparisonOpr,
        final_bound: ExprIdx,
        range: TextRange,
    ) -> AstResult<Self> {
        todo!()
        // let final_boundary_kind = match comparison {
        //     // ill-formed: $frame_var >= $final_bound
        //     BinaryComparisonOpr::Geq => err!("invalid form", range)?,
        //     // ill-formed: $frame_var > $final_bound
        //     BinaryComparisonOpr::Greater => err!("invalid form", range)?,
        //     // well-formed: $frame_var <= $final_bound
        //     BinaryComparisonOpr::Leq => BoundaryKind::UpperClosed,
        //     // well-formed: $frame_var < $final_bound
        //     BinaryComparisonOpr::Less => BoundaryKind::UpperOpen,
        //     _ => todo!(),
        // };
        // Ok(Self::For {
        //     frame_var,
        //     initial_boundary: Default::default(),
        //     final_boundary: RawBoundary {
        //         opt_bound: Some(final_bound),
        //         kind: final_boundary_kind,
        //     },
        //     step: LoopStep(1),
        // })
    }

    pub fn for_loop_with_default_final(
        initial_bound: ExprIdx,
        comparison: BinaryComparisonOpr,
        frame_var: RangedIdentifier,
        range: TextRange,
    ) -> AstResult<Self> {
        todo!()
        // let initial_boundary_kind = match comparison {
        //     // well-formed: $initial_bound >= $frame_var
        //     BinaryComparisonOpr::Geq => BoundaryKind::LowerClosed,
        //     // well-formed: $initial_bound > $frame_var
        //     BinaryComparisonOpr::Greater => BoundaryKind::LowerOpen,
        //     // ill-formed: $initial_bound <= $frame_var
        //     BinaryComparisonOpr::Leq => err!("invalid form", range)?,
        //     // ill-formed: $initial_bound < $frame_var
        //     BinaryComparisonOpr::Less => err!("invalid form", range)?,
        //     _ => return err!(format!("expect comparison"), range),
        // };
        // Ok(Self::For {
        //     frame_var,
        //     initial_boundary: RawBoundary {
        //         opt_bound: Some(initial_bound),
        //         kind: initial_boundary_kind,
        //     },
        //     final_boundary: Default::default(),
        //     step: LoopStep(-1),
        // })
    }

    pub fn forext_loop(
        frame_var: RangedIdentifier,
        comparison: BinaryComparisonOpr,
        bound: ExprIdx,
        range: TextRange,
    ) -> AstResult<Self> {
        todo!()
        // let (boundary_kind, step) = match comparison {
        //     // ... $frame_var >= $final_bound
        //     BinaryComparisonOpr::Geq => (BoundaryKind::LowerClosed, LoopStep(-1)),
        //     // ... $frame_var > $final_bound
        //     BinaryComparisonOpr::Greater => (BoundaryKind::LowerOpen, LoopStep(-1)),
        //     // ... $frame_var <= $final_bound
        //     BinaryComparisonOpr::Leq => (BoundaryKind::UpperClosed, LoopStep(1)),
        //     // ... $frame_var < $final_bound
        //     BinaryComparisonOpr::Less => (BoundaryKind::UpperOpen, LoopStep(1)),
        //     _ => return err!(format!("expect comparison"), range),
        // };
        // Ok(Self::ForExt {
        //     frame_var,
        //     final_boundary: RawBoundary {
        //         opt_bound: Some(bound),
        //         kind: boundary_kind,
        //     },
        //     step,
        // })
    }

    pub fn while_loop(condition: ExprIdx) -> Self {
        Self::While { condition }
    }

    pub fn do_while_loop(condition: ExprIdx) -> Self {
        Self::DoWhile { condition }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RawBoundary {
    pub opt_bound: Option<ExprIdx>,
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
