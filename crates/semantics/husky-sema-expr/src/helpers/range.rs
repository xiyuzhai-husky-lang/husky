use super::*;
use husky_print_utils::p;
use husky_regional_token::{RegionalTokenIdxRange, RegionalTokenIdxRangeEnd};
use husky_vfs::ModulePath;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb, jar = SemaExprJar)]
pub struct SemaExprRangeRegion {
    item_path_expr_ranges: Vec<RegionalTokenIdxRange>,
    pattern_expr_ranges: Vec<RegionalTokenIdxRange>,
    expr_ranges: Vec<RegionalTokenIdxRange>,
    stmt_ranges: SemaStmtMap<RegionalTokenIdxRange>,
}

#[salsa::tracked(jar = SemaExprJar, return_ref)]
pub(crate) fn sema_expr_range_region(
    db: &dyn SemaExprDb,
    expr_region: SemaExprRegion,
) -> SemaExprRangeRegion {
    SemaExprRangeCalculator::new(db, expr_region).calc_all()
}

// #[test]
// fn expr_range_sheet_works() {
//     use tests::*;
//     DB::default().ast_expect_test_debug_with_db("expr_range_sheet", todo!());
// }

impl std::ops::Index<SynPrincipalEntityPathExprIdx> for SemaExprRangeRegion {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynPrincipalEntityPathExprIdx) -> &Self::Output {
        &self.item_path_expr_ranges[index.index()]
    }
}

impl std::ops::Index<SynPatternExprIdx> for SemaExprRangeRegion {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynPatternExprIdx) -> &Self::Output {
        &self.pattern_expr_ranges[index.index()]
    }
}

impl std::ops::Index<SemaExprIdx> for SemaExprRangeRegion {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SemaExprIdx) -> &Self::Output {
        &self.expr_ranges[index.index()]
    }
}

struct SemaExprRangeCalculator<'a> {
    expr_region_data: &'a SemaExprRegionData,
    principal_entity_path_expr_ranges: Vec<RegionalTokenIdxRange>,
    pattern_expr_ranges: Vec<RegionalTokenIdxRange>,
    expr_ranges: Vec<RegionalTokenIdxRange>,
    stmt_ranges: SemaStmtMap<RegionalTokenIdxRange>,
}

impl<'a> std::ops::Index<SynPrincipalEntityPathExprIdx> for SemaExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynPrincipalEntityPathExprIdx) -> &Self::Output {
        &self.principal_entity_path_expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<&SynPrincipalEntityPathExprIdx> for SemaExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: &SynPrincipalEntityPathExprIdx) -> &Self::Output {
        &self.principal_entity_path_expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<SynPatternExprIdx> for SemaExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynPatternExprIdx) -> &Self::Output {
        &self.pattern_expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<SemaExprIdx> for SemaExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SemaExprIdx) -> &Self::Output {
        &self.expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<&SemaExprIdx> for SemaExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: &SemaExprIdx) -> &Self::Output {
        &self.expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<SemaStmtIdx> for SemaExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SemaStmtIdx) -> &Self::Output {
        &self.stmt_ranges[index]
    }
}

impl<'a> SemaExprRangeCalculator<'a> {
    fn new(db: &'a dyn SemaExprDb, expr_region: SemaExprRegion) -> Self {
        let expr_region_data = expr_region.data(db);
        let region_path = expr_region_data.path();
        SemaExprRangeCalculator {
            expr_region_data,
            principal_entity_path_expr_ranges: Default::default(),
            pattern_expr_ranges: Default::default(),
            expr_ranges: Default::default(),
            stmt_ranges: SemaStmtMap::new(expr_region_data.stmt_arena()),
        }
    }

    fn calc_all(mut self) -> SemaExprRangeRegion {
        // order matters
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
        SemaExprRangeRegion {
            item_path_expr_ranges: self.principal_entity_path_expr_ranges,
            pattern_expr_ranges: self.pattern_expr_ranges,
            expr_ranges: self.expr_ranges,
            stmt_ranges: self.stmt_ranges,
        }
    }

    // fn calc_principal_entity_path_expr_range(
    //     &self,
    //     expr: &SemaPrincipalEntityPathExpr,
    // ) -> RegionalTokenIdxRange {
    //     match expr {
    //         SemaPrincipalEntityPathExpr::Root {
    //             path_name_token,
    //             principal_entity_path: item_path,
    //         } => RegionalTokenIdxRange::new_single(path_name_token.regional_token_idx()),
    //         SemaPrincipalEntityPathExpr::Subitem {
    //             parent,
    //             colon_colon_token,
    //             ident_token,
    //             path,
    //         } => match ident_token {
    //             Ok(ident_token) => self[parent].to(RegionalTokenIdxRangeEnd::new_after(
    //                 ident_token.regional_token_idx(),
    //             )),
    //             Err(_) => self[parent].to(RegionalTokenIdxRangeEnd::new_after(
    //                 colon_colon_token.regional_token_idx(),
    //             )),
    //         },
    //     }
    // }

    fn calc_expr_range(&mut self, expr: &SemaExprData) -> RegionalTokenIdxRange {
        match expr {
            SemaExprData::Literal(regional_token_idx, _)
            | SemaExprData::InheritedSymbol {
                regional_token_idx, ..
            }
            | SemaExprData::CurrentSymbol {
                regional_token_idx, ..
            }
            | SemaExprData::FrameVarDecl {
                regional_token_idx, ..
            }
            | SemaExprData::SelfType(regional_token_idx)
            | SemaExprData::SelfValue(regional_token_idx) => {
                RegionalTokenIdxRange::new_single(*regional_token_idx)
            }
            SemaExprData::Binary { lopd, ropd, .. } => self[lopd].join(self[ropd]),
            SemaExprData::PrincipalEntityPath { path_expr_idx, .. } => self[*path_expr_idx],
            SemaExprData::AssociatedItem {
                parent_expr_idx,
                ident_token,
                ..
            } => {
                // todo: consider implicit(angular) arguments
                self[parent_expr_idx].to(RegionalTokenIdxRangeEnd::new_after(
                    ident_token.regional_token_idx(),
                ))
            }
            SemaExprData::Be {
                src,
                be_regional_token_idx,
                target,
            } => {
                let start = self[src].start().regional_token_idx();
                let end = if let Ok(target) = target {
                    self[target.sema_pattern_root().sema_pattern_expr_idx()].end()
                } else {
                    RegionalTokenIdxRangeEnd::new_after(*be_regional_token_idx)
                };
                RegionalTokenIdxRange::new(start, end)
            }
            SemaExprData::Prefix {
                opr,
                opr_regional_token_idx,
                opd_sema_expr_idx,
            } => RegionalTokenIdxRange::new(*opr_regional_token_idx, self[opd_sema_expr_idx].end()),
            SemaExprData::Suffix {
                opd_sema_expr_idx,
                opr,
                opr_regional_token_idx,
            } => self[opd_sema_expr_idx]
                .to(RegionalTokenIdxRangeEnd::new_after(*opr_regional_token_idx)),
            SemaExprData::FunctionApplicationOrCall {
                function: first_expr,
                rpar_regional_token_idx,
                ..
            }
            | SemaExprData::FunctionCall {
                function: first_expr,
                rpar_regional_token_idx,
                ..
            }
            | SemaExprData::MethodApplicationOrCall {
                self_argument: first_expr,
                rpar_regional_token_idx,
                ..
            } => self[first_expr].to(RegionalTokenIdxRangeEnd::new_after(
                *rpar_regional_token_idx,
            )),
            SemaExprData::Field {
                owner_sema_expr_idx,
                ident_token,
                ..
            } => self[owner_sema_expr_idx].to(RegionalTokenIdxRangeEnd::new_after(
                ident_token.regional_token_idx(),
            )),
            SemaExprData::TemplateInstantiation { template, .. } => todo!(),
            SemaExprData::ExplicitApplication {
                function_expr_idx: function,
                argument_expr_idx: argument,
            } => self[function].join(self[argument]),
            SemaExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => match place_label_regional_token {
                Some(_) => todo!(),
                None => RegionalTokenIdxRange::new_single(*at_regional_token_idx),
            },
            SemaExprData::Unit {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
                ..
            }
            | SemaExprData::Bracketed {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
                ..
            }
            | SemaExprData::NewTuple {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
                ..
            } => RegionalTokenIdxRange::new(
                *lpar_regional_token_idx,
                RegionalTokenIdxRangeEnd::new_after(*rpar_regional_token_idx),
            ),
            SemaExprData::IndexOrCompositionWithList {
                owner,
                lbox_regional_token_idx,
                rbox_regional_token_idx,
                ..
            } => self[owner].to(RegionalTokenIdxRangeEnd::new_after(
                *rbox_regional_token_idx,
            )),
            SemaExprData::List {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
                ..
            } => RegionalTokenIdxRange::new(
                *lbox_regional_token_idx,
                RegionalTokenIdxRangeEnd::new_after(*rbox_regional_token_idx),
            ),
            SemaExprData::BoxColonList {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
                ..
            } => RegionalTokenIdxRange::new(
                *lbox_regional_token_idx,
                RegionalTokenIdxRangeEnd::new_after(*rbox_regional_token_idx),
            ),
            SemaExprData::Block { stmts } => self.calc_block_range(*stmts),
            SemaExprData::EmptyHtmlTag {
                empty_html_bra_idx,
                empty_html_ket,
                ..
            } => RegionalTokenIdxRange::new_closed(
                *empty_html_bra_idx,
                empty_html_ket.regional_token_idx(),
            ),
            SemaExprData::Ritchie {
                ritchie_kind_regional_token_idx,
                rpar_regional_token_idx,
                return_ty_sema_expr_idx: return_ty_expr,
                ..
            } => match return_ty_expr {
                Some(return_ty_expr) => RegionalTokenIdxRange::new(
                    *ritchie_kind_regional_token_idx,
                    self[*return_ty_expr].end(),
                ),
                None => RegionalTokenIdxRange::new_closed(
                    *ritchie_kind_regional_token_idx,
                    *rpar_regional_token_idx,
                ),
            },
            SemaExprData::Sorry { regional_token_idx }
            | SemaExprData::Todo { regional_token_idx }
            | SemaExprData::Unreachable { regional_token_idx } => {
                RegionalTokenIdxRange::new_single(*regional_token_idx)
            }
            SemaExprData::Err(error) => match error {
                SemaExprDataError::Original(error) => error.regional_token_idx_range(),
                SemaExprDataError::Derived(_) => todo!(),
            },
        }
    }

    fn calc_block_range(&mut self, stmts: SemaStmtIdxRange) -> RegionalTokenIdxRange {
        for stmt in stmts {
            self.save_stmt_range(stmt);
        }
        self[stmts.start()].join(self[stmts.end() - 1])
    }

    fn save_stmt_range(&mut self, stmt_idx: SemaStmtIdx) {
        let range = self.calc_stmt_range(stmt_idx);
        // after calculation, all the child statements must have already been computed and cached
        // so that self.stmt_ranges.len() is equal to stmt_idx.raw()
        self.stmt_ranges.insert_new(stmt_idx, range)
    }

    fn calc_stmt_range(&mut self, stmt_idx: SemaStmtIdx) -> RegionalTokenIdxRange {
        match self.expr_region_data[stmt_idx] {
            SemaStmtData::Let {
                let_token,
                ref let_variables_pattern,
                ref assign_token,
                ref initial_value, /* todo: other types of let initialization */
                ..
            } => {
                let start = let_token.regional_token_idx();
                let end = self[initial_value].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemaStmtData::Return {
                return_token,
                ref result,
            } => {
                let start = return_token.regional_token_idx();
                let end = self[result].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemaStmtData::Require {
                require_token,
                ref condition,
            } => {
                let start = require_token.regional_token_idx();
                let end = self[condition].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemaStmtData::Assert {
                assert_token,
                ref condition,
            } => {
                let start = assert_token.regional_token_idx();
                let end = self[condition].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemaStmtData::Break { break_token } => {
                RegionalTokenIdxRange::new_single(break_token.regional_token_idx())
            }
            SemaStmtData::Eval { expr_idx, .. } => self[expr_idx],
            SemaStmtData::ForBetween {
                for_token,
                ref particulars,
                ref eol_colon,
                ref block,
                ..
            } => {
                let start = for_token.regional_token_idx();
                let end = self.calc_block_range(*block).end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemaStmtData::ForIn {
                for_token,
                ref block,
                ..
            } => todo!(),
            SemaStmtData::ForExt {
                forext_token,
                /* todo: particulars */
                ref eol_colon,
                ref block,
                ..
            } => {
                let start = forext_token.regional_token_idx();
                let end = self.calc_block_range(*block).end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemaStmtData::While {
                while_token,
                ref condition,
                ref eol_colon,
                ref block,
                ..
            } => {
                let start = while_token.regional_token_idx();
                let end = self.calc_block_range(*block).end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemaStmtData::DoWhile {
                do_token,
                ref condition,
                ref eol_colon,
                ref block,
                ..
            } => {
                let start = do_token.regional_token_idx();
                let end = self.calc_block_range(*block).end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemaStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => {
                let start = if_branch.if_token.regional_token_idx();
                // it's important that every branch is computed
                let if_branch_end: RegionalTokenIdxRangeEnd =
                    self.calc_block_range(if_branch.stmts()).end();
                let mut elif_branch_rev_iter = elif_branches.iter().rev();
                let elif_branches_end: Option<RegionalTokenIdxRangeEnd> = {
                    if let Some(last_elif_branch) = elif_branch_rev_iter.next() {
                        Some(self.calc_block_range(last_elif_branch.stmts()).end())
                    } else {
                        None
                    }
                };
                for elif_branch in elif_branch_rev_iter {
                    self.calc_block_range(elif_branch.stmts());
                }
                let else_block_end: Option<RegionalTokenIdxRangeEnd> =
                    if let Some(else_branch) = else_branch {
                        Some(self.calc_block_range(else_branch.stmts()).end())
                    } else {
                        None
                    };
                let end = else_block_end
                    .or(elif_branches_end)
                    .unwrap_or(if_branch_end);
                RegionalTokenIdxRange::new(start, end)
            }
            SemaStmtData::Match { match_token, .. } => {
                // ad hoc
                RegionalTokenIdxRange::new_single(match_token.regional_token_idx())
            }
        }
    }
}
