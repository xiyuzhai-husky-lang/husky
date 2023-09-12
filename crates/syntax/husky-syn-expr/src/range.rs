use crate::*;
use husky_print_utils::p;
use husky_vfs::ModulePath;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb, jar = SynExprJar)]
pub struct ExprRangeRegion {
    item_path_expr_ranges: Vec<RegionalTokenIdxRange>,
    pattern_expr_ranges: Vec<RegionalTokenIdxRange>,
    expr_ranges: Vec<RegionalTokenIdxRange>,
    stmt_ranges: SynStmtMap<RegionalTokenIdxRange>,
}

#[salsa::tracked(jar = SynExprJar, return_ref)]
pub(crate) fn expr_range_region(db: &dyn SynExprDb, expr_region: SynExprRegion) -> ExprRangeRegion {
    SynExprRangeCalculator::new(db, expr_region).calc_all()
}

// #[test]
// fn expr_range_sheet_works() {
//     use tests::*;
//     DB::default().ast_expect_test_debug_with_db("expr_range_sheet", todo!());
// }

impl std::ops::Index<PrincipalEntityPathExprIdx> for ExprRangeRegion {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: PrincipalEntityPathExprIdx) -> &Self::Output {
        &self.item_path_expr_ranges[index.index()]
    }
}

impl std::ops::Index<SynPatternExprIdx> for ExprRangeRegion {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynPatternExprIdx) -> &Self::Output {
        &self.pattern_expr_ranges[index.index()]
    }
}

impl std::ops::Index<SynExprIdx> for ExprRangeRegion {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynExprIdx) -> &Self::Output {
        &self.expr_ranges[index.index()]
    }
}

struct SynExprRangeCalculator<'a> {
    expr_region_data: &'a SynExprRegionData,
    item_path_expr_ranges: Vec<RegionalTokenIdxRange>,
    pattern_expr_ranges: Vec<RegionalTokenIdxRange>,
    expr_ranges: Vec<RegionalTokenIdxRange>,
    stmt_ranges: SynStmtMap<RegionalTokenIdxRange>,
}

impl<'a> std::ops::Index<PrincipalEntityPathExprIdx> for SynExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: PrincipalEntityPathExprIdx) -> &Self::Output {
        &self.item_path_expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<&PrincipalEntityPathExprIdx> for SynExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: &PrincipalEntityPathExprIdx) -> &Self::Output {
        &self.item_path_expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<SynPatternExprIdx> for SynExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynPatternExprIdx) -> &Self::Output {
        &self.pattern_expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<SynExprIdx> for SynExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynExprIdx) -> &Self::Output {
        &self.expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<&SynExprIdx> for SynExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: &SynExprIdx) -> &Self::Output {
        &self.expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<SynStmtIdx> for SynExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynStmtIdx) -> &Self::Output {
        &self.stmt_ranges[index]
    }
}

impl<'a> SynExprRangeCalculator<'a> {
    fn new(db: &'a dyn SynExprDb, expr_region: SynExprRegion) -> Self {
        let expr_region_data = expr_region.data(db);
        let region_path = expr_region_data.path();
        SynExprRangeCalculator {
            expr_region_data,
            item_path_expr_ranges: Default::default(),
            pattern_expr_ranges: Default::default(),
            expr_ranges: Default::default(),
            stmt_ranges: SynStmtMap::new(expr_region_data.stmt_arena()),
        }
    }

    fn calc_all(mut self) -> ExprRangeRegion {
        // order matters
        self.item_path_expr_ranges
            .reserve(self.expr_region_data.principal_item_path_expr_arena().len());
        for item_path_expr in self
            .expr_region_data
            .principal_item_path_expr_arena()
            .iter()
        {
            self.item_path_expr_ranges
                .push(self.calc_item_path_expr_range(item_path_expr))
        }
        self.pattern_expr_ranges
            .reserve(self.expr_region_data.pattern_expr_arena().len());
        for pattern_expr in self.expr_region_data.pattern_expr_arena().iter() {
            self.pattern_expr_ranges
                .push(self.calc_pattern_expr_range(pattern_expr))
        }
        self.expr_ranges
            .reserve(self.expr_region_data.expr_arena().len());
        for expr in self.expr_region_data.expr_arena().iter() {
            let expr_range = self.calc_expr_range(expr);
            self.expr_ranges.push(expr_range)
        }
        assert_eq!(
            self.expr_region_data.expr_arena().len(),
            self.expr_ranges.len()
        );
        ExprRangeRegion {
            item_path_expr_ranges: self.item_path_expr_ranges,
            pattern_expr_ranges: self.pattern_expr_ranges,
            expr_ranges: self.expr_ranges,
            stmt_ranges: self.stmt_ranges,
        }
    }

    fn calc_item_path_expr_range(&self, expr: &PrincipalEntityPathExpr) -> RegionalTokenIdxRange {
        match expr {
            PrincipalEntityPathExpr::Root {
                path_name_token,
                principal_entity_path: item_path,
            } => RegionalTokenIdxRange::new_single(path_name_token.regional_token_idx()),
            PrincipalEntityPathExpr::Subitem {
                parent,
                colon_colon_token,
                ident_token,
                path,
            } => match ident_token {
                Ok(ident_token) => self[parent].to(RegionalTokenIdxRangeEnd::new_after(
                    ident_token.regional_token_idx(),
                )),
                Err(_) => self[parent].to(RegionalTokenIdxRangeEnd::new_after(
                    colon_colon_token.token_idx(),
                )),
            },
        }
    }

    fn calc_pattern_expr_range(&self, expr: &SynPatternExpr) -> RegionalTokenIdxRange {
        match expr {
            SynPatternExpr::Literal(_) => todo!(),
            SynPatternExpr::Ident {
                symbol_modifier_keyword_group,
                ident_token,
            } => match symbol_modifier_keyword_group {
                Some(EphemSymbolModifierRegionalTokenGroup::Mut(mut_token)) => {
                    RegionalTokenIdxRange::new_closed(
                        mut_token.token_idx(),
                        ident_token.regional_token_idx(),
                    )
                }
                Some(EphemSymbolModifierRegionalTokenGroup::RefMut(ref_token, ..)) => {
                    RegionalTokenIdxRange::new_closed(
                        ref_token.token_idx(),
                        ident_token.regional_token_idx(),
                    )
                }
                Some(_) => todo!(),
                None => RegionalTokenIdxRange::new_single(ident_token.regional_token_idx()),
            },
            SynPatternExpr::Entity(_) => todo!(),
            SynPatternExpr::Tuple { name, fields } => todo!(),
            SynPatternExpr::Props { name, fields } => todo!(),
            SynPatternExpr::OneOf { options } => todo!(),
            SynPatternExpr::Binding {
                ident_token,
                asperand_token,
                src,
            } => todo!(),
            SynPatternExpr::Range {
                start,
                dot_dot_token,
                end,
            } => todo!(),
        }
    }

    fn calc_expr_range(&mut self, expr: &SynExpr) -> RegionalTokenIdxRange {
        match expr {
            SynExpr::Literal(token_idx, _)
            | SynExpr::InheritedSymbol { token_idx, .. }
            | SynExpr::CurrentSymbol { token_idx, .. }
            | SynExpr::FrameVarDecl { token_idx, .. }
            | SynExpr::SelfType(token_idx)
            | SynExpr::SelfValue(token_idx) => RegionalTokenIdxRange::new_single(*token_idx),
            SynExpr::Binary { lopd, ropd, .. } => self[lopd].join(self[ropd]),
            SynExpr::PrincipalEntityPath {
                item_path_expr,
                opt_path: item_path,
            } => self[*item_path_expr],
            SynExpr::ScopeResolution {
                parent_expr_idx,
                colon_colon_regional_token,
                ident_token,
            } => {
                // todo: consider implicit(angular) arguments
                self[parent_expr_idx].to(RegionalTokenIdxRangeEnd::new_after(
                    ident_token.regional_token_idx(),
                ))
            }
            SynExpr::Be {
                src,
                be_token_idx,
                target,
            } => {
                let start = self[src].start().token_idx();
                let end = if let Ok(target) = target {
                    self[target.pattern_expr()].end()
                } else {
                    RegionalTokenIdxRangeEnd::new_after(*be_token_idx)
                };
                RegionalTokenIdxRange::new(start, end)
            }
            SynExpr::Prefix {
                opr,
                opr_token_idx,
                opd,
            } => RegionalTokenIdxRange::new(*opr_token_idx, self[opd].end()),
            SynExpr::Suffix {
                opd,
                opr,
                opr_token_idx,
            } => self[opd].to(RegionalTokenIdxRangeEnd::new_after(*opr_token_idx)),
            SynExpr::FunctionApplicationOrCall {
                function: first_expr,
                rpar_token_idx,
                ..
            }
            | SynExpr::FunctionCall {
                function: first_expr,
                rpar_token_idx,
                ..
            }
            | SynExpr::MethodApplicationOrCall {
                self_argument: first_expr,
                rpar_token_idx,
                ..
            } => self[first_expr].to(RegionalTokenIdxRangeEnd::new_after(*rpar_token_idx)),
            SynExpr::Field {
                owner, ident_token, ..
            } => self[owner].to(RegionalTokenIdxRangeEnd::new_after(
                ident_token.regional_token_idx(),
            )),
            SynExpr::TemplateInstantiation {
                template,
                generic_arguments,
            } => todo!(),
            SynExpr::ExplicitApplication {
                function_expr_idx: function,
                argument_expr_idx: argument,
            } => self[function].join(self[argument]),
            SynExpr::Unit {
                lpar_token_idx,
                rpar_token_idx,
                ..
            }
            | SynExpr::Bracketed {
                lpar_token_idx,
                rpar_token_idx,
                ..
            }
            | SynExpr::NewTuple {
                lpar_token_idx,
                rpar_token_idx,
                ..
            } => RegionalTokenIdxRange::new(
                *lpar_token_idx,
                RegionalTokenIdxRangeEnd::new_after(*rpar_token_idx),
            ),
            SynExpr::IndexOrCompositionWithList {
                owner,
                lbox_token_idx,
                rbox_token_idx,
                ..
            } => self[owner].to(RegionalTokenIdxRangeEnd::new_after(*rbox_token_idx)),
            SynExpr::List {
                lbox_token_idx,
                rbox_token_idx,
                ..
            } => RegionalTokenIdxRange::new(
                *lbox_token_idx,
                RegionalTokenIdxRangeEnd::new_after(*rbox_token_idx),
            ),
            SynExpr::BoxColonList {
                lbox_token_idx,
                rbox_token_idx,
                ..
            } => RegionalTokenIdxRange::new(
                *lbox_token_idx,
                RegionalTokenIdxRangeEnd::new_after(*rbox_token_idx),
            ),
            SynExpr::Block { stmts } => self.calc_block_range(*stmts),
            SynExpr::EmptyHtmlTag {
                empty_html_bra_idx,
                empty_html_ket,
                ..
            } => RegionalTokenIdxRange::new_closed(*empty_html_bra_idx, empty_html_ket.token_idx()),
            SynExpr::Ritchie {
                ritchie_kind_token_idx,
                rpar_token_idx,
                return_ty_expr,
                ..
            } => match return_ty_expr {
                Some(return_ty_expr) => {
                    RegionalTokenIdxRange::new(*ritchie_kind_token_idx, self[*return_ty_expr].end())
                }
                None => RegionalTokenIdxRange::new_closed(*ritchie_kind_token_idx, *rpar_token_idx),
            },
            SynExpr::Sorry {
                regional_token_idx: token_idx,
            } => todo!(),
            SynExpr::Todo {
                regional_token_idx: token_idx,
            } => RegionalTokenIdxRange::new_single(*token_idx),
            SynExpr::Err(error) => match error {
                SynExprError::Original(error) => error.token_idx_range(),
                SynExprError::Derived(_) => todo!(),
            },
        }
    }

    fn calc_block_range(&mut self, stmts: SynStmtIdxRange) -> RegionalTokenIdxRange {
        for stmt in stmts {
            self.save_stmt_range(stmt);
        }
        self[stmts.start()].join(self[stmts.end() - 1])
    }

    fn save_stmt_range(&mut self, stmt_idx: SynStmtIdx) {
        let range = self.calc_stmt_range(stmt_idx);
        // after calculation, all the child statements must have already been computed and cached
        // so that self.stmt_ranges.len() is equal to stmt_idx.raw()
        self.stmt_ranges.insert_new(stmt_idx, range)
    }

    fn calc_stmt_range(&mut self, stmt_idx: SynStmtIdx) -> RegionalTokenIdxRange {
        match self.expr_region_data[stmt_idx] {
            SynStmt::Let {
                let_token,
                ref let_variables_pattern,
                ref assign_token,
                ref initial_value, /* todo: other types of let initialization */
                ..
            } => {
                let start = let_token.token_idx();
                let end = self[initial_value].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmt::Return {
                return_token,
                ref result,
            } => {
                let start = return_token.token_idx();
                let end = self[result].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmt::Require {
                require_token,
                ref condition,
            } => {
                let start = require_token.token_idx();
                let end = self[condition].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmt::Assert {
                assert_token,
                ref condition,
            } => {
                let start = assert_token.token_idx();
                let end = self[condition].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmt::Break { break_token } => {
                RegionalTokenIdxRange::new_single(break_token.token_idx())
            }
            SynStmt::Eval { expr_idx, .. } => self[expr_idx],
            SynStmt::ForBetween {
                for_token,
                ref particulars,
                ref eol_colon,
                ref block,
                ..
            } => {
                let start = for_token.token_idx();
                let end = if let Ok(block) = block {
                    self.calc_block_range(*block).end()
                } else if let Ok(eol_colon) = eol_colon {
                    RegionalTokenIdxRangeEnd::new_after(eol_colon.token_idx())
                } else {
                    match particulars.range {
                        Ok(ref range) => {
                            if let Some(bound_expr) = range.final_boundary.bound_expr {
                                self[bound_expr].end()
                            } else {
                                RegionalTokenIdxRangeEnd::new_after(
                                    particulars.for_between_loop_var_token_idx,
                                )
                            }
                        }
                        Err(_) => todo!(),
                    }
                };
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmt::ForIn {
                for_token,
                ref block,
                ..
            } => todo!(),
            SynStmt::ForExt {
                forext_token,
                /* todo: particulars */
                ref eol_colon,
                ref block,
                ..
            } => {
                let start = forext_token.token_idx();
                let end = if let Ok(block) = block {
                    self.calc_block_range(*block).end()
                } else if let Ok(eol_colon) = eol_colon {
                    RegionalTokenIdxRangeEnd::new_after(eol_colon.token_idx())
                } else {
                    /* todo: particulars */
                    RegionalTokenIdxRangeEnd::new_after(start)
                };
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmt::While {
                while_token,
                ref condition,
                ref eol_colon,
                ref block,
                ..
            } => {
                let start = while_token.token_idx();
                let end = if let Ok(block) = block {
                    self.calc_block_range(*block).end()
                } else if let Ok(eol_colon) = eol_colon {
                    RegionalTokenIdxRangeEnd::new_after(eol_colon.token_idx())
                } else if let Ok(condition) = condition {
                    self[condition].end()
                } else {
                    RegionalTokenIdxRangeEnd::new_after(start)
                };
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmt::DoWhile {
                do_token,
                ref condition,
                ref eol_colon,
                ref block,
                ..
            } => {
                let start = do_token.token_idx();
                let end = if let Ok(block) = block {
                    self.calc_block_range(*block).end()
                } else if let Ok(eol_colon) = eol_colon {
                    RegionalTokenIdxRangeEnd::new_after(eol_colon.token_idx())
                } else if let Ok(condition) = condition {
                    self[condition].end()
                } else {
                    RegionalTokenIdxRangeEnd::new_after(start)
                };
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmt::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => {
                let start = if_branch.if_token.token_idx();
                // it's important that every branch is computed
                let if_branch_end: RegionalTokenIdxRangeEnd = if let Ok(block) = if_branch.stmts() {
                    self.calc_block_range(block).end()
                } else if let Ok(eol_colon_token) = if_branch.eol_colon_token() {
                    RegionalTokenIdxRangeEnd::new_after(eol_colon_token.token_idx())
                } else if let Ok(condition) = if_branch.condition {
                    self[condition].end()
                } else {
                    RegionalTokenIdxRangeEnd::new_after(if_branch.if_token.token_idx())
                };
                let mut elif_branch_rev_iter = elif_branches.iter().rev();
                let elif_branches_end: Option<RegionalTokenIdxRangeEnd> = {
                    if let Some(last_elif_branch) = elif_branch_rev_iter.next() {
                        if let Ok(block) = last_elif_branch.stmts() {
                            Some(self.calc_block_range(block).end())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };
                for elif_branch in elif_branch_rev_iter {
                    if let Ok(block) = elif_branch.stmts() {
                        self.calc_block_range(block);
                    }
                }
                let else_block_end: Option<RegionalTokenIdxRangeEnd> =
                    if let Some(else_branch) = else_branch {
                        if let Ok(block) = else_branch.stmts() {
                            Some(self.calc_block_range(block).end())
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                let end = else_block_end
                    .or(elif_branches_end)
                    .unwrap_or(if_branch_end);
                RegionalTokenIdxRange::new(start, end)
            }
            SynStmt::Match { match_token } => {
                // ad hoc
                RegionalTokenIdxRange::new_single(match_token.token_idx())
            }
        }
    }
}
