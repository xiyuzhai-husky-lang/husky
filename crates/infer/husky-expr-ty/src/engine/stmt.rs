use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_block(
        &mut self,
        stmts: StmtIdxRange,
        expr_expectation: ExprTypeExpectation,
    ) -> Option<LocalTerm> {
        for stmt in stmts.start()..(stmts.end() - 1) {
            self.infer_new_nonlast_stmt(stmt)
        }
        self.infer_new_last_stmt(stmts.end() - 1, expr_expectation)
    }
    fn visit_new_exprs_in_stmt(&mut self, stmt_idx: StmtIdx) {}

    fn infer_new_nonlast_stmt(&mut self, stmt_idx: StmtIdx) {
        let expect_unit = self.expect_unit();
        match self.calc_stmt(stmt_idx, expect_unit) {
            Some(ty) => match ty {
                LocalTerm::Resolved(ty) => match ty {
                    ty if ty == self.reduced_term_menu.unit() => return,
                    ty => todo!(),
                },
                LocalTerm::Unresolved(_) => todo!(),
            },
            None => (),
        }
    }

    fn expect_unit(&mut self) -> ExprTypeExpectation {
        ExprTypeExpectation::ImplicitlyConvertibleTo {
            ty: self.reduced_term_menu.unit().into(),
        }
    }

    fn infer_new_last_stmt(
        &mut self,
        stmt_idx: StmtIdx,
        expr_expectation: ExprTypeExpectation,
    ) -> Option<LocalTerm> {
        self.calc_stmt(stmt_idx, expr_expectation)
    }

    fn calc_stmt(
        &mut self,
        stmt_idx: StmtIdx,
        expr_expectation: ExprTypeExpectation,
    ) -> Option<LocalTerm> {
        match self.expr_region_data[stmt_idx] {
            Stmt::Let {
                let_token,
                ref let_variable_pattern,
                ref initial_value,
                ..
            } => self.calc_let_stmt(let_variable_pattern, initial_value),
            Stmt::Return { ref result, .. } => {
                if let Ok(result) = result {
                    self.infer_new_expr_ty(
                        *result,
                        ExprTypeExpectation::Return { ty: self.return_ty },
                    );
                };
                Some(self.reduced_term_menu.never().into())
            }
            Stmt::Require { ref condition, .. } => {
                if let Ok(condition) = condition {
                    self.infer_new_expr_ty(*condition, ExprTypeExpectation::CastibleAsBool);
                };
                Some(self.reduced_term_menu.unit().into())
            }
            Stmt::Assert { ref condition, .. } => {
                if let Ok(condition) = condition {
                    self.infer_new_expr_ty(*condition, ExprTypeExpectation::CastibleAsBool);
                };
                Some(self.reduced_term_menu.unit().into())
            }
            Stmt::Break { .. } => Some(self.reduced_term_menu.never().into()),
            Stmt::Eval { expr_idx } => self.infer_new_expr_ty(expr_idx, expr_expectation),
            Stmt::ForBetween {
                ref particulars,
                ref block,
                ..
            } => {
                let mut expected_frame_var_ty: Option<LocalTerm> = None;
                if let Some(bound_expr) = particulars.range.initial_boundary.bound_expr {
                    match self.infer_new_expr_ty(bound_expr, ExprTypeExpectation::None) {
                        Some(_) => todo!(),
                        None => todo!(),
                    }
                }
                if let Some(bound_expr) = particulars.range.final_boundary.bound_expr {
                    let expectation = match expected_frame_var_ty {
                        Some(_) => todo!(),
                        None => ExprTypeExpectation::None,
                    };
                    match self.infer_new_expr_ty(bound_expr, expectation) {
                        Some(ty) if expected_frame_var_ty == None => {
                            expected_frame_var_ty = Some(ty)
                        }
                        _ => (),
                    }
                }
                if let Ok(block) = block {
                    let expr_expectation = self.expect_unit();
                    self.infer_new_block(*block, expr_expectation);
                }
                Some(self.reduced_term_menu.unit().into())
            }
            Stmt::ForIn {
                ref condition,
                ref block,
                ..
            } => todo!(),
            Stmt::ForExt { ref block, .. } => todo!(),
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
                    self.infer_new_expr_ty(condition, ExprTypeExpectation::CastibleAsBool)
                });
                block.as_ref().copied().map(|block| {
                    let expect_unit = self.expect_unit();
                    self.infer_new_block(block, expect_unit)
                });
                Some(self.reduced_term_menu.unit().into())
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
            Stmt::Match {} => todo!(),
            Stmt::Err(_) => todo!(),
        }
    }

    fn calc_let_stmt(
        &mut self,
        let_variable_pattern: &Result<LetVariablesPattern, ExprError>,
        initial_value: &Result<ExprIdx, ExprError>,
    ) -> Option<LocalTerm> {
        let pattern_ty = match let_variable_pattern {
            Ok(pattern) => match pattern.ty() {
                Some(ty) => todo!(),
                None => {
                    initial_value
                        .as_ref()
                        .ok()
                        .map(|initial_value| {
                            self.infer_new_expr_ty(
                                *initial_value,
                                // ad hoc
                                ExprTypeExpectation::None,
                            )
                        })
                        .flatten()
                }
            },
            Err(_) => todo!(),
        };
        match pattern_ty {
            Some(ty) if ty == self.reduced_term_menu.never().into() => {
                Some(self.reduced_term_menu.never().into())
            }
            Some(ty) => {
                match let_variable_pattern {
                    Ok(let_variable_pattern) => self.infer_pattern_and_symbols_ty(
                        let_variable_pattern.pattern_expr(),
                        ty,
                        let_variable_pattern.variables(),
                    ),
                    Err(_) => todo!(),
                };
                Some(self.reduced_term_menu.unit().into())
            }
            None => Some(self.reduced_term_menu.unit().into()),
        }
    }

    fn infer_pattern_and_symbols_ty(
        &mut self,
        pattern_expr_idx: PatternExprIdx,
        ty: LocalTerm,
        symbols: CurrentSymbolIdxRange,
    ) {
        self.save_pattern_ty(pattern_expr_idx, ty);
        for symbol in symbols {
            self.infer_new_current_symbol_ty(symbol)
        }
    }

    /// the way type inference works for pattern expressions is dual to that of regular expression
    fn save_pattern_ty(&mut self, pattern_expr_idx: PatternExprIdx, ty: LocalTerm) {
        self.pattern_expr_ty_infos
            .insert_new(pattern_expr_idx, PatternExprTypeInfo::new(Ok(ty)));
        self.infer_subpattern_tys(pattern_expr_idx)
    }

    /// subpattern expressions get its type from its parent
    fn infer_subpattern_tys(&mut self, pattern_expr_idx: PatternExprIdx) {
        match self.expr_region_data[pattern_expr_idx] {
            PatternExpr::Literal(_) => todo!(),
            PatternExpr::Identifier { .. } => (), // there is no subpattern to infer
            PatternExpr::Entity(_) => todo!(),
            PatternExpr::Tuple { name, fields } => todo!(),
            PatternExpr::Struct { name, fields } => todo!(),
            PatternExpr::OneOf { options } => todo!(),
            PatternExpr::Binding {
                ident_token,
                asperand_token,
                src,
            } => todo!(),
            PatternExpr::Range {
                start,
                dot_dot_token,
                end,
            } => todo!(),
        }
    }

    fn infer_new_current_symbol_ty(&mut self, current_symbol_idx: CurrentSymbolIdx) {
        if let Some(ty) = self.calc_new_current_symbol_ty(current_symbol_idx) {
            self.current_symbol_tys.insert_new(current_symbol_idx, ty)
        }
    }

    fn calc_new_current_symbol_ty(
        &mut self,
        current_symbol_idx: idx_arena::ArenaIdx<CurrentSymbol>,
    ) -> Option<LocalTerm> {
        match self.expr_region_data[current_symbol_idx].variant() {
            CurrentSymbolVariant::ImplicitParameter {
                implicit_parameter_variant,
            } => todo!(),
            CurrentSymbolVariant::RegularParameter { pattern_symbol_idx } => todo!(),
            CurrentSymbolVariant::LetVariable { pattern_symbol_idx } => {
                self.infer_new_pattern_symbol_ty(*pattern_symbol_idx)
            }
            CurrentSymbolVariant::FrameVariable(_) => todo!(),
        }
    }

    fn infer_new_pattern_symbol_ty(
        &mut self,
        pattern_symbol_idx: PatternSymbolIdx,
    ) -> Option<LocalTerm> {
        let ty_result = self.calc_new_pattern_symbol_ty(pattern_symbol_idx);
        let ty = ty_result.as_ref().ok().copied();
        self.pattern_symbol_ty_infos
            .insert_new(pattern_symbol_idx, PatternSymbolTypeInfo::new(ty_result));
        ty
    }

    fn calc_new_pattern_symbol_ty(
        &mut self,
        pattern_symbol_idx: PatternSymbolIdx,
    ) -> PatternSymbolTypeResult<LocalTerm> {
        match self.expr_region_data[pattern_symbol_idx] {
            PatternSymbol::Atom(pattern_expr_idx) => self
                .get_pattern_expr_ty(pattern_expr_idx)
                .ok_or(DerivedPatternSymbolTypeError::PatternExprTypeError.into()),
        }
    }

    fn get_pattern_expr_ty(&self, pattern_expr_idx: PatternExprIdx) -> Option<LocalTerm> {
        self.pattern_expr_ty_infos
            .get(pattern_expr_idx)
            .map(|info| info.ty().ok().copied())
            .flatten()
    }

    fn calc_if_else_stmt(
        &mut self,
        if_branch: &IfBranch,
        elif_branches: &[ElifBranch],
        else_branch: Option<&ElseBranch>,
        expr_expectation: ExprTypeExpectation,
    ) -> Option<LocalTerm> {
        let mut branch_tys = BranchTypes::new(expr_expectation);
        if_branch.condition.as_ref().copied().map(|condition| {
            self.infer_new_expr_ty(condition, ExprTypeExpectation::CastibleAsBool)
        });
        branch_tys.visit_branch(self, &if_branch.block);
        for elif_branch in elif_branches {
            elif_branch.condition.as_ref().copied().map(|condition| {
                self.infer_new_expr_ty(condition, ExprTypeExpectation::CastibleAsBool)
            });
            branch_tys.visit_branch(self, &elif_branch.block);
        }
        if let Some(else_branch) = else_branch {
            branch_tys.visit_branch(self, &else_branch.block);
        }
        // exhaustive iff else branch exists
        branch_tys.merge(else_branch.is_some())
    }
}

struct BranchTypes {
    /// this is true if the type of one of the branches cannot be inferred
    has_error: bool,
    /// this is true if the type of one of the branches is inferred to be not never
    has_ever: bool,
    expr_expectation: ExprTypeExpectation,
}

impl BranchTypes {
    fn new(expr_expectation: ExprTypeExpectation) -> Self {
        Self {
            has_error: false,
            has_ever: false,
            expr_expectation,
        }
    }

    fn visit_branch(&mut self, engine: &mut ExprTypeEngine, block: &ExprResult<StmtIdxRange>) {
        match block {
            Ok(stmts) => match engine.infer_new_block(*stmts, self.expr_expectation) {
                Some(LocalTerm::Resolved(term)) if term == engine.reduced_term_menu.never() => (),
                Some(term) => {
                    p!(term.debug(engine.db));
                    todo!()
                }
                None => self.has_error = true,
            },
            Err(_) => self.has_error = true,
        };
    }

    fn merge(self, exhaustive: bool) -> Option<LocalTerm> {
        if self.has_error {
            return None;
        }
        if self.has_ever {
            return todo!();
        }
        todo!()
    }
}
