use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub struct SynForBetweenParticulars {
    pub frame_var_token_idx: TokenIdx,
    pub frame_var_expr_idx: SynExprIdx,
    pub frame_var_ident: Ident,
    pub range: SynForBetweenRange,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub struct SynForBetweenRange {
    pub initial_boundary: SynLoopBoundary,
    pub final_boundary: SynLoopBoundary,
    pub step: LoopStep,
}

impl SynForBetweenRange {
    pub fn new_without_defaults(
        initial_bound: SynExprIdx,
        initial_comparison: BinaryComparisonOpr,
        final_comparison: BinaryComparisonOpr,
        final_bound: SynExprIdx,
    ) -> StmtResult<Self> {
        let (initial_boundary_kind, step) = match initial_comparison {
            BinaryComparisonOpr::Geq => (LoopBoundaryKind::UpperClosed, LoopStep::Constant(-1)),
            BinaryComparisonOpr::Greater => (LoopBoundaryKind::UpperOpen, LoopStep::Constant(-1)),
            BinaryComparisonOpr::Leq => (LoopBoundaryKind::LowerClosed, LoopStep::Constant(1)),
            BinaryComparisonOpr::Less => (LoopBoundaryKind::LowerOpen, LoopStep::Constant(1)),
            _ => todo!(),
        };
        let final_boundary_kind = match final_comparison {
            // ... $frame_var >= $final_bound
            BinaryComparisonOpr::Geq => LoopBoundaryKind::LowerClosed,
            // ... $frame_var > $final_bound
            BinaryComparisonOpr::Greater => LoopBoundaryKind::LowerOpen,
            // ... $frame_var <= $final_bound
            BinaryComparisonOpr::Leq => LoopBoundaryKind::UpperClosed,
            // ... $frame_var < $final_bound
            BinaryComparisonOpr::Less => LoopBoundaryKind::UpperOpen,
            _ => todo!(),
        };
        check_compatible(initial_boundary_kind, final_boundary_kind)?;
        Ok(Self {
            initial_boundary: SynLoopBoundary {
                bound_expr: Some(initial_bound),
                kind: initial_boundary_kind,
            },
            final_boundary: SynLoopBoundary {
                bound_expr: Some(final_bound),
                kind: final_boundary_kind,
            },
            step,
        })
    }

    pub(crate) fn new_with_default_initial(
        comparison: BinaryComparisonOpr,
        final_bound: SynExprIdx,
    ) -> StmtResult<Self> {
        let final_boundary_kind = match comparison {
            // ill-formed: $frame_var >= $final_bound
            BinaryComparisonOpr::Geq => todo!("invalid form",),
            // ill-formed: $frame_var > $final_bound
            BinaryComparisonOpr::Greater => todo!("invalid form",),
            // well-formed: $frame_var <= $final_bound
            BinaryComparisonOpr::Leq => LoopBoundaryKind::UpperClosed,
            // well-formed: $frame_var < $final_bound
            BinaryComparisonOpr::Less => LoopBoundaryKind::UpperOpen,
            _ => todo!(),
        };
        Ok(SynForBetweenRange {
            initial_boundary: Default::default(),
            final_boundary: SynLoopBoundary {
                bound_expr: Some(final_bound),
                kind: final_boundary_kind,
            },
            step: LoopStep::Constant(1),
        })
    }

    pub(crate) fn new_with_default_final(
        initial_bound: SynExprIdx,
        comparison: BinaryComparisonOpr,
    ) -> StmtResult<Self> {
        let initial_boundary_kind = match comparison {
            // well-formed: $initial_bound >= $frame_var
            BinaryComparisonOpr::Geq => LoopBoundaryKind::LowerClosed,
            // well-formed: $initial_bound > $frame_var
            BinaryComparisonOpr::Greater => LoopBoundaryKind::LowerOpen,
            // ill-formed: $initial_bound <= $frame_var
            BinaryComparisonOpr::Leq => todo!("invalid form",),
            // ill-formed: $initial_bound < $frame_var
            BinaryComparisonOpr::Less => todo!("invalid form",),
            _ => return todo!("expect comparison"),
        };
        Ok(Self {
            initial_boundary: SynLoopBoundary {
                bound_expr: Some(initial_bound),
                kind: initial_boundary_kind,
            },
            final_boundary: Default::default(),
            step: LoopStep::Constant(-1),
        })
    }
}

fn check_compatible(
    initial_boundary_kind: LoopBoundaryKind,
    final_boundary_kind: LoopBoundaryKind,
) -> StmtResult<()> {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Direction {
        Incremental,
        Decremental,
    }

    let initial_direction = match initial_boundary_kind {
        LoopBoundaryKind::UpperOpen | LoopBoundaryKind::UpperClosed => Direction::Decremental,
        LoopBoundaryKind::LowerOpen | LoopBoundaryKind::LowerClosed => Direction::Incremental,
    };
    let final_direction = match final_boundary_kind {
        LoopBoundaryKind::UpperOpen | LoopBoundaryKind::UpperClosed => Direction::Incremental,
        LoopBoundaryKind::LowerOpen | LoopBoundaryKind::LowerClosed => Direction::Decremental,
    };
    if initial_direction == final_direction {
        Ok(())
    } else {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SynLoopBoundary {
    pub bound_expr: Option<SynExprIdx>,
    pub kind: LoopBoundaryKind,
}

impl Default for SynLoopBoundary {
    fn default() -> Self {
        Self {
            bound_expr: None,
            kind: LoopBoundaryKind::LowerClosed,
        }
    }
}

/// loop boundary kind
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LoopBoundaryKind {
    UpperOpen,
    UpperClosed,
    LowerOpen,
    LowerClosed,
}

/// loop step
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LoopStep {
    Constant(i32),
}

pub struct LoopStepValue(pub i32);

impl LoopStepValue {
    pub fn n(&self, a: i32, b: i32) -> i32 {
        if (b - a) * self.0 >= 0 {
            (b - a) / self.0 + 1
        } else {
            0
        }
    }

    pub fn frame_var(&self, a: i32, i: i32) -> i32 {
        a + self.0 * i
    }
}

#[test]
fn test_step_n_for_pos_step() {
    let step = LoopStepValue(1);
    assert_eq!(step.n(0, 0), 1);
    assert_eq!(step.n(0, 1), 2);
    assert_eq!(step.n(0, 2), 3);
    assert_eq!(step.n(0, -1), 0);
}

#[test]
fn test_step_n_for_neg_step() {
    let step = LoopStepValue(-1);
    assert_eq!(step.n(0, 0), 1);
    assert_eq!(step.n(0, -1), 2);
    assert_eq!(step.n(0, -2), 3);
    assert_eq!(step.n(0, -3), 4);
    assert_eq!(step.n(0, 1), 0);
}
