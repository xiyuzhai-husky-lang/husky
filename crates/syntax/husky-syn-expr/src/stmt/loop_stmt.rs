use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub struct SynForBetweenParticulars {
    pub for_between_loop_var_regional_token_idx: RegionalTokenIdx,
    pub for_between_loop_var_ident: Ident,
    pub for_between_loop_var_expr_idx: SynExprIdx,
    pub range: SynExprResult<SynForBetweenRange>,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub struct SynForBetweenRange {
    pub initial_boundary: SynForBetweenLoopBoundary,
    pub final_boundary: SynForBetweenLoopBoundary,
    pub step: LoopStep,
}

impl SynForBetweenRange {
    pub fn new_without_defaults(
        initial_bound: SynExprIdx,
        initial_comparison: BinaryComparisonOpr,
        final_comparison: BinaryComparisonOpr,
        final_bound: SynExprIdx,
    ) -> SynExprResult<Self> {
        let (initial_boundary_kind, step) = match initial_comparison {
            BinaryComparisonOpr::Geq => (LoopBoundaryKind::UpperClosed, LoopStep::Constant(-1)),
            BinaryComparisonOpr::Greater => (LoopBoundaryKind::UpperOpen, LoopStep::Constant(-1)),
            BinaryComparisonOpr::Leq => (LoopBoundaryKind::LowerClosed, LoopStep::Constant(1)),
            BinaryComparisonOpr::Less => (LoopBoundaryKind::LowerOpen, LoopStep::Constant(1)),
            _ => todo!(),
        };
        let final_boundary_kind = LoopBoundaryKind::new_final(final_comparison);
        Self::check_for_between_range_compatibility(initial_boundary_kind, final_boundary_kind)?;
        Ok(Self {
            initial_boundary: SynForBetweenLoopBoundary {
                bound_expr: Some(initial_bound),
                kind: initial_boundary_kind,
            },
            final_boundary: SynForBetweenLoopBoundary {
                bound_expr: Some(final_bound),
                kind: final_boundary_kind,
            },
            step,
        })
    }

    pub(crate) fn new_with_default_initial(
        comparison: BinaryComparisonOpr,
        final_bound: SynExprIdx,
    ) -> Self {
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
        SynForBetweenRange {
            initial_boundary: Default::default(),
            final_boundary: SynForBetweenLoopBoundary {
                bound_expr: Some(final_bound),
                kind: final_boundary_kind,
            },
            step: LoopStep::Constant(1),
        }
    }

    pub(crate) fn new_with_default_final(
        initial_bound: SynExprIdx,
        comparison: BinaryComparisonOpr,
    ) -> Self {
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
        Self {
            initial_boundary: SynForBetweenLoopBoundary {
                bound_expr: Some(initial_bound),
                kind: initial_boundary_kind,
            },
            final_boundary: Default::default(),
            step: LoopStep::Constant(-1),
        }
    }

    fn check_for_between_range_compatibility(
        initial_boundary_kind: LoopBoundaryKind,
        final_boundary_kind: LoopBoundaryKind,
    ) -> SynExprResult<()> {
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
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SynForBetweenLoopBoundary {
    pub bound_expr: Option<SynExprIdx>,
    pub kind: LoopBoundaryKind,
}

impl Default for SynForBetweenLoopBoundary {
    fn default() -> Self {
        Self {
            bound_expr: None,
            kind: LoopBoundaryKind::LowerClosed,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SynForextParticulars {
    pub forext_loop_var_regional_token_idx: RegionalTokenIdx,
    pub forext_loop_var_ident: Ident,
    pub forext_loop_var_expr_idx: SynExprIdx,
    pub bound_expr: SynExprIdx,
    pub boundary_kind: LoopBoundaryKind,
}

impl SynForextParticulars {
    pub(crate) fn new(
        forext_loop_var_regional_token_idx: RegionalTokenIdx,
        forext_loop_var_ident: Ident,
        forext_loop_var_expr_idx: SynExprIdx,
        opr: BinaryComparisonOpr,
        bound_expr: SynExprIdx,
    ) -> Self {
        Self {
            forext_loop_var_regional_token_idx,
            forext_loop_var_ident,
            forext_loop_var_expr_idx,
            bound_expr,
            boundary_kind: LoopBoundaryKind::new_final(opr),
        }
    }
}

/// loop boundary kind
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LoopBoundaryKind {
    UpperOpen,
    UpperClosed,
    LowerOpen,
    LowerClosed,
}

impl LoopBoundaryKind {
    fn new_final(final_comparison: BinaryComparisonOpr) -> LoopBoundaryKind {
        match final_comparison {
            // ... $frame_var >= $final_bound
            BinaryComparisonOpr::Geq => LoopBoundaryKind::LowerClosed,
            // ... $frame_var > $final_bound
            BinaryComparisonOpr::Greater => LoopBoundaryKind::LowerOpen,
            // ... $frame_var <= $final_bound
            BinaryComparisonOpr::Leq => LoopBoundaryKind::UpperClosed,
            // ... $frame_var < $final_bound
            BinaryComparisonOpr::Less => LoopBoundaryKind::UpperOpen,
            _ => todo!(),
        }
    }
}

/// loop step
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

impl<'a> SynStmtContext<'a> {
    pub(super) fn parse_for_loop_stmt(
        &mut self,
        token_group_idx: RegionalTokenGroupIdx,
        for_token: StmtForRegionalToken,
        expr: SynExprIdx,
        eol_colon: SynExprResult<EolRegionalToken>,
        body: DefnAstIdxRange,
    ) -> SynStmt {
        match self.syn_expr_arena()[expr] {
            SynExpr::Binary {
                lopd,
                opr: BinaryOpr::Comparison(comparison_opr),
                opr_regional_token_idx,
                ropd,
            } => {
                let particulars = self.parse_for_between_particulars(lopd, ropd, comparison_opr);
                let current_symbol_variant = CurrentSynSymbolVariant::FrameVariable {
                    expr_idx: particulars.for_between_loop_var_expr_idx,
                    ident: particulars.for_between_loop_var_ident,
                };
                let current_symbol_kind = current_symbol_variant.kind();
                let access_start = self
                    .regional_ast_token_idx_range(body.start())
                    .start()
                    .regional_token_idx();
                let access_end = self.regional_ast_token_idx_range(body.end() - 1).end();
                let frame_var_symbol = CurrentSynSymbol::new(
                    self.syn_pattern_expr_region(),
                    access_start,
                    Some(access_end),
                    current_symbol_variant,
                );
                let frame_var_symbol_idx = self
                    .define_symbols(
                        vec![frame_var_symbol],
                        Some(ObeliskTypeConstraint::FrameVariable),
                    )
                    .start();
                self.syn_expr_arena_mut().set(
                    particulars.for_between_loop_var_expr_idx,
                    SynExpr::FrameVarDecl {
                        regional_token_idx: particulars.for_between_loop_var_regional_token_idx,
                        ident: particulars.for_between_loop_var_ident,
                        frame_var_symbol_idx,
                        current_symbol_kind,
                    },
                );
                SynStmt::ForBetween {
                    for_token,
                    particulars,
                    frame_var_symbol_idx,
                    eol_colon,
                    block: self.parse_stmts_expected(body, token_group_idx),
                }
            }
            SynExpr::Binary {
                lopd,
                opr: BinaryOpr::In,
                opr_regional_token_idx,
                ropd,
            } => SynStmt::ForIn {
                for_token,
                condition: todo!(),
                eol_colon,
                block: self.parse_stmts_expected(body, token_group_idx),
            },
            _ => todo!(),
        }
    }

    fn parse_for_between_particulars(
        &self,
        lopd: SynExprIdx,
        ropd: SynExprIdx,
        comparison_opr: BinaryComparisonOpr,
    ) -> SynForBetweenParticulars {
        use OriginalSynExprError::UnrecognizedIdent;
        let lopd_expr = &self.syn_expr_arena()[lopd];
        let ropd_expr = &self.syn_expr_arena()[ropd];
        // todo: parse with
        if let SynExpr::Err(SynExprError::Original(UnrecognizedIdent {
            regional_token_idx,
            ident,
        })) = lopd_expr
        {
            SynForBetweenParticulars {
                for_between_loop_var_regional_token_idx: *regional_token_idx,
                for_between_loop_var_expr_idx: lopd,
                for_between_loop_var_ident: *ident,
                range: Ok(SynForBetweenRange::new_with_default_initial(
                    comparison_opr,
                    ropd,
                )),
            }
            // SynExpr::Err(SynExprError::Original(UnrecognizedIdent {..})) will be changed to Ok
        } else if let SynExpr::Err(SynExprError::Original(UnrecognizedIdent {
            regional_token_idx,
            ident,
        })) = ropd_expr
        {
            SynForBetweenParticulars {
                for_between_loop_var_regional_token_idx: *regional_token_idx,
                for_between_loop_var_expr_idx: ropd,
                for_between_loop_var_ident: *ident,
                range: Ok(SynForBetweenRange::new_with_default_final(
                    lopd,
                    comparison_opr,
                )),
            }
        } else {
            let final_comparison = comparison_opr;
            match lopd_expr {
                SynExpr::Binary {
                    lopd: llopd,
                    opr: BinaryOpr::Comparison(initial_comparison),
                    opr_regional_token_idx,
                    ropd: lropd,
                } => {
                    let lropd_expr = &self.syn_expr_arena()[lropd];
                    match lropd_expr {
                        SynExpr::Err(SynExprError::Original(UnrecognizedIdent {
                            regional_token_idx,
                            ident,
                        })) => SynForBetweenParticulars {
                            for_between_loop_var_regional_token_idx: *regional_token_idx,
                            for_between_loop_var_expr_idx: *lropd,
                            for_between_loop_var_ident: *ident,
                            range: SynForBetweenRange::new_without_defaults(
                                *llopd,
                                *initial_comparison,
                                final_comparison,
                                ropd,
                            ),
                        },
                        _ => todo!(),
                    }
                }
                _ => todo!(),
            }
        }
    }

    pub(super) fn parse_forext_loop_stmt(
        &mut self,
        token_group_idx: RegionalTokenGroupIdx,
        forext_token: ForextRegionalToken,
        expr: SynExprIdx,
        eol_colon: SynExprResult<EolRegionalToken>,
        body: DefnAstIdxRange,
    ) -> SynStmt {
        let SynExpr::Binary {
            lopd: forext_loop_var_expr_idx,
            opr: BinaryOpr::Comparison(opr),
            opr_regional_token_idx,
            ropd: bound_expr,
        } = self.syn_expr_arena()[expr]
        else {
            todo!()
        };
        let (forext_loop_var_ident, forext_loop_var_regional_token_idx) =
            match self.syn_expr_arena()[forext_loop_var_expr_idx] {
                SynExpr::InheritedSymbol {
                    ident,
                    regional_token_idx,
                    inherited_symbol_idx,
                    inherited_symbol_kind,
                } => (ident, regional_token_idx),
                SynExpr::CurrentSymbol {
                    ident,
                    regional_token_idx,
                    current_symbol_idx,
                    current_symbol_kind,
                } => (ident, regional_token_idx),
                _ => todo!(),
            };
        let particulars = SynForextParticulars::new(
            forext_loop_var_regional_token_idx,
            forext_loop_var_ident,
            forext_loop_var_expr_idx,
            opr,
            bound_expr,
        );
        SynStmt::ForExt {
            forext_token,
            particulars,
            eol_colon,
            block: self.parse_stmts_expected(body, token_group_idx),
        }
    }
}
