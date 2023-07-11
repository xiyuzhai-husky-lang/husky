mod let_init;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_block(
        &mut self,
        stmts: StmtIdxRange,
        expr_expectation: impl ExpectFluffyTerm,
    ) -> Option<FluffyTerm> {
        for stmt in stmts.start()..(stmts.end() - 1) {
            self.infer_new_nonlast_stmt(stmt)
        }
        self.infer_new_last_stmt(stmts.end() - 1, expr_expectation)
    }

    fn infer_new_nonlast_stmt(&mut self, stmt_idx: StmtIdx) {
        let expect_unit = self.expect_unit();
        self.calc_stmt(stmt_idx, expect_unit);
    }

    fn infer_new_last_stmt(
        &mut self,
        stmt_idx: StmtIdx,
        expr_expectation: impl ExpectFluffyTerm,
    ) -> Option<FluffyTerm> {
        self.calc_stmt(stmt_idx, expr_expectation)
    }

    fn calc_stmt(
        &mut self,
        stmt_idx: StmtIdx,
        expr_expectation: impl ExpectFluffyTerm,
    ) -> Option<FluffyTerm> {
        match self.expr_region_data[stmt_idx] {
            Stmt::Let {
                let_token,
                ref let_variable_pattern,
                initial_value,
                ..
            } => self.calc_let_stmt(let_variable_pattern, initial_value),
            Stmt::Return { result, .. } => {
                match self.return_ty {
                    Some(return_ty) => {
                        self.infer_new_expr_ty_discarded(
                            result,
                            ExpectCoersion::new_move(return_ty.into()),
                        );
                    }
                    None => {
                        self.infer_new_expr_ty_discarded(result, ExpectAnyDerived);
                    }
                };
                Some(self.term_menu.never().into())
            }
            Stmt::Require { condition, .. } => {
                self.infer_new_expr_ty_discarded(condition, self.expect_argument_ty_bool());
                Some(self.term_menu.unit_ty_ontology().into())
            }
            Stmt::Assert { condition, .. } => {
                self.infer_new_expr_ty_discarded(condition, self.expect_argument_ty_bool());
                Some(self.term_menu.unit_ty_ontology().into())
            }
            Stmt::Break { .. } => Some(self.term_menu.never().into()),
            Stmt::Eval { expr_idx } => self.infer_new_expr_ty(expr_idx, expr_expectation),
            Stmt::ForBetween {
                ref particulars,
                frame_var_symbol_idx,
                ref block,
                ..
            } => {
                let mut expected_frame_var_ty: Option<FluffyTerm> = None;
                if let Some(bound_expr) = particulars.range.initial_boundary.bound_expr {
                    match self.infer_new_expr_ty_for_outcome(bound_expr, ExpectNumType) {
                        Some(num_ty_outcome) => {
                            expected_frame_var_ty = Some(num_ty_outcome.placeless_num_ty())
                        }
                        None => (),
                    }
                }
                if let Some(bound_expr) = particulars.range.final_boundary.bound_expr {
                    match expected_frame_var_ty {
                        Some(expected_frame_var_ty) => {
                            self.infer_new_expr_ty_discarded(
                                bound_expr,
                                ExpectCoersion::new_pure(self, expected_frame_var_ty),
                            );
                        }
                        None => {
                            if let Some(ty) = self.infer_new_expr_ty(bound_expr, ExpectAnyOriginal)
                            {
                                expected_frame_var_ty = Some(ty)
                            }
                        }
                    }
                }
                if let Some(expected_frame_var_ty) = expected_frame_var_ty {
                    let place = Place::ImmutableStackOwned {
                        location: frame_var_symbol_idx
                            .into_local_symbol_idx(self.expr_region_data)
                            .into(),
                    };
                    let frame_var_symbol_ty =
                        SymbolType::new(self, frame_var_symbol_idx, expected_frame_var_ty);
                    self.symbol_tys
                        .insert_new(frame_var_symbol_idx, frame_var_symbol_ty)
                }
                if let Ok(block) = block {
                    let expr_expectation = self.expect_unit();
                    self.infer_new_block(*block, expr_expectation);
                }
                Some(self.term_menu.unit_ty_ontology().into())
            }
            Stmt::ForIn {
                ref condition,
                ref block,
                ..
            } => todo!(),
            Stmt::ForExt { ref block, .. } => {
                // ad hoc: handle for ext particulars
                if let Ok(block) = block {
                    let expr_expectation = self.expect_unit();
                    self.infer_new_block(*block, expr_expectation);
                }
                Some(self.term_menu.unit_ty_ontology().into())
            }
            Stmt::While {
                ref condition,
                ref block,
                ..
            }
            | Stmt::DoWhile {
                ref condition,
                ref block,
                ..
            } => {
                condition.as_ref().copied().map(|condition| {
                    self.infer_new_expr_ty_discarded(condition, self.expect_argument_ty_bool())
                });
                block.as_ref().copied().map(|block| {
                    let expect_unit = self.expect_unit();
                    self.infer_new_block(block, expect_unit)
                });
                Some(self.term_menu.unit_ty_ontology().into())
            }
            Stmt::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => self.calc_if_else_stmt(
                if_branch,
                elif_branches,
                else_branch.as_ref(),
                expr_expectation,
            ),
            Stmt::Match { .. } => {
                // todo: match
                None
            }
            Stmt::Err(_) => todo!(),
        }
    }

    fn calc_if_else_stmt(
        &mut self,
        if_branch: &IfBranch,
        elif_branches: &[ElifBranch],
        else_branch: Option<&ElseBranch>,
        expr_expectation: impl ExpectFluffyTerm,
    ) -> Option<FluffyTerm> {
        let mut branch_tys = BranchTypes::new(expr_expectation);
        if_branch
            .condition
            .as_ref()
            .copied()
            .map(|condition| self.infer_new_expr_ty(condition, self.expect_argument_ty_bool()));
        branch_tys.visit_branch(self, &if_branch.block);
        for elif_branch in elif_branches {
            elif_branch.condition.as_ref().copied().map(|condition| {
                self.infer_new_expr_ty_discarded(condition, self.expect_argument_ty_bool())
            });
            branch_tys.visit_branch(self, &elif_branch.block);
        }
        if let Some(else_branch) = else_branch {
            branch_tys.visit_branch(self, &else_branch.block);
        }
        // exhaustive iff else branch exists
        branch_tys.merge(else_branch.is_some(), &self.term_menu)
    }
}

struct BranchTypes<Expectation: ExpectFluffyTerm> {
    /// this is true if the type of one of the branches cannot be inferred
    has_error: bool,
    /// this is true if the type of one of the branches is inferred to be not never
    ever_ty: Option<FluffyTerm>,
    expr_expectation: Expectation,
}

impl<Expectation: ExpectFluffyTerm> BranchTypes<Expectation> {
    fn new(expr_expectation: Expectation) -> Self {
        Self {
            has_error: false,
            ever_ty: None,
            expr_expectation,
        }
    }

    fn visit_branch(&mut self, engine: &mut ExprTypeEngine, block: &ExprResult<StmtIdxRange>) {
        match block {
            Ok(stmts) => match engine.infer_new_block(*stmts, self.expr_expectation.clone()) {
                Some(FluffyTerm::EntityPath(TermEntityPath::TypeOntology(path)))
                    if path == engine.entity_path_menu.never_ty_path() =>
                {
                    ()
                }
                Some(new_block_ty) => {
                    if self.ever_ty.is_none() {
                        self.ever_ty = Some(new_block_ty)
                    }
                }
                None => self.has_error = true,
            },
            Err(_) => self.has_error = true,
        };
    }

    fn merge(self, exhaustive: bool, menu: &EtherealTermMenu) -> Option<FluffyTerm> {
        if let Some(ever_ty) = self.ever_ty {
            return ever_ty.into();
        }
        (!self.has_error).then_some(menu.never().into())
    }
}
