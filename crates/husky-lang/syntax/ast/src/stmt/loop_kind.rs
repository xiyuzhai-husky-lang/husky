use vm::{BoundaryKind, LoopStep, PureBinaryOpr};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RawLoopKind {
    For {
        frame_var: CustomIdentifier,
        initial_boundary: RawBoundary,
        final_boundary: RawBoundary,
        step: LoopStep,
    },
    ForExt {
        bound: RawExprIdx,
        is_shifted: bool,
        is_incremental: bool,
        fvar_ident: CustomIdentifier,
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
        frame_var: CustomIdentifier,
        final_comparison: PureBinaryOpr,
        final_bound: RawExprIdx,
        range: TextRange,
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
        frame_var: CustomIdentifier,
        comparison: PureBinaryOpr,
        final_bound: RawExprIdx,
        range: TextRange,
    ) -> AstResult<Self> {
        let final_boundary_kind = match comparison {
            // ill-formed: $frame_var >= $final_bound
            PureBinaryOpr::Geq => err!(range, "invalid form")?,
            // ill-formed: $frame_var > $final_bound
            PureBinaryOpr::Greater => err!(range, "invalid form")?,
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
        pure_binary: PureBinaryOpr,
        frame_var: CustomIdentifier,
        range: TextRange,
    ) -> AstResult<Self> {
        let initial_boundary_kind = match pure_binary {
            // well-formed: $initial_bound >= $frame_var
            PureBinaryOpr::Geq => BoundaryKind::LowerClosed,
            // well-formed: $initial_bound > $frame_var
            PureBinaryOpr::Greater => BoundaryKind::LowerOpen,
            // ill-formed: $initial_bound <= $frame_var
            PureBinaryOpr::Leq => err!(range, "invalid form")?,
            // ill-formed: $initial_bound < $frame_var
            PureBinaryOpr::Less => err!(range, "invalid form")?,
            _ => todo!(),
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
    // match pure_binary {
    //     PureBinaryOpr::Geq => todo!(),
    //     PureBinaryOpr::Greater => todo!(),
    //     PureBinaryOpr::Leq => todo!(),
    //     PureBinaryOpr::Less => {
    //         let frame_var = match lopd.kind {
    //             RawExprKind::Variable(ident) => ident,
    //             RawExprKind::Unrecognized(_) => todo!(),
    //             RawExprKind::Scope { scope, kind } => todo!(),
    //             RawExprKind::Literal(_) => todo!(),
    //             RawExprKind::Bracketed(_) => todo!(),
    //             RawExprKind::Opn { opr, ref opds } => todo!(),
    //             RawExprKind::Lambda(_, _) => todo!(),
    //         };
    //         todo!()
    //     }
    //     _ => todo!(),
    // }
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
