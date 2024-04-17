use self::condition::SemaCondition;

use super::*;
use husky_print_utils::p;
use husky_regional_token::{
    EphemSymbolModifierRegionalTokens, RegionalTokenIdxRange, RegionalTokenIdxRangeEnd,
};
use husky_syn_expr::entity_path::{PrincipalEntityPathSynExprIdx, SynPrincipalEntityPathExpr};
use husky_vfs::ModulePath;

#[salsa::tracked(db = SemaExprDb, jar = SemaExprJar)]
pub struct SemaExprRangeRegion {
    #[return_ref]
    pub data: SemaExprRangeRegionData,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SemaExprRangeRegionData {
    principal_entity_path_expr_ranges: Vec<RegionalTokenIdxRange>,
    pattern_expr_ranges: Vec<RegionalTokenIdxRange>,
    expr_ranges: Vec<RegionalTokenIdxRange>,
    // use SemaStmtMap because the inference order may not be in consistent with stmt id
    stmt_ranges: SemaStmtMap<RegionalTokenIdxRange>,
}

#[salsa::tracked(jar = SemaExprJar)]
pub fn sem_expr_range_region(
    db: &::salsa::Db,
    sem_expr_region: SemaExprRegion,
) -> SemaExprRangeRegion {
    SemaExprRangeRegion::new(
        db,
        SemaExprRangeCalculator::new(db, sem_expr_region).calc_all(),
    )
}

#[cfg(test)]
fn decl_sem_expr_range_regions(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> Vec<SemaExprRangeRegion> {
    use husky_syn_decl::HasSynDeclSheet;

    module_path
        .syn_decl_sheet(db)
        .decls(db)
        .iter()
        .copied()
        .filter_map(|(_, decl)| {
            Some(sem_expr_range_region(
                db,
                db.sem_expr_region(decl.syn_expr_region(db)?),
            ))
        })
        .collect()
}

#[test]
fn decl_sem_expr_range_regions_works() {
    use tests::*;
    DB::ast_expect_test_debug_with_db(
        decl_sem_expr_range_regions,
        &AstTestConfig::new(
            "decl_sem_expr_range_regions",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    );
}

#[cfg(test)]
fn defn_sem_expr_range_regions(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> Vec<SemaExprRangeRegion> {
    use husky_syn_defn::module_item_syn_defns;
    module_item_syn_defns(db, module_path)
        .into_iter()
        .filter_map(|(_, defn)| {
            Some(sem_expr_range_region(
                db,
                db.sem_expr_region(defn?.syn_expr_region),
            ))
        })
        .collect()
}

#[test]
fn defn_sem_expr_range_regions_works() {
    use tests::*;
    DB::ast_expect_test_debug_with_db(
        defn_sem_expr_range_regions,
        &AstTestConfig::new(
            "defn_sem_expr_range_regions",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    );
}

impl std::ops::Index<PrincipalEntityPathSynExprIdx> for SemaExprRangeRegionData {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: PrincipalEntityPathSynExprIdx) -> &Self::Output {
        &self.principal_entity_path_expr_ranges[index.index()]
    }
}

impl std::ops::Index<SynPatternIdx> for SemaExprRangeRegionData {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynPatternIdx) -> &Self::Output {
        &self.pattern_expr_ranges[index.index()]
    }
}

impl std::ops::Index<SemaExprIdx> for SemaExprRangeRegionData {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SemaExprIdx) -> &Self::Output {
        &self.expr_ranges[index.index()]
    }
}

impl std::ops::Index<SemaStmtIdx> for SemaExprRangeRegionData {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SemaStmtIdx) -> &Self::Output {
        &self.stmt_ranges[index]
    }
}

struct SemaExprRangeCalculator<'a> {
    syn_expr_region_data: &'a SynExprRegionData,
    sem_expr_region_data: &'a SemaExprRegionData,
    principal_entity_path_expr_ranges: Vec<RegionalTokenIdxRange>,
    pattern_expr_ranges: Vec<RegionalTokenIdxRange>,
    expr_ranges: Vec<RegionalTokenIdxRange>,
    stmt_ranges: SemaStmtMap<RegionalTokenIdxRange>,
}

impl<'a> std::ops::Index<PrincipalEntityPathSynExprIdx> for SemaExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: PrincipalEntityPathSynExprIdx) -> &Self::Output {
        &self.principal_entity_path_expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<&PrincipalEntityPathSynExprIdx> for SemaExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: &PrincipalEntityPathSynExprIdx) -> &Self::Output {
        &self.principal_entity_path_expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<SynPatternIdx> for SemaExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynPatternIdx) -> &Self::Output {
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
    fn new(db: &'a ::salsa::Db, sem_expr_region: SemaExprRegion) -> Self {
        let syn_expr_region = sem_expr_region.syn_expr_region(db);
        let syn_expr_region_data = syn_expr_region.data(db);
        let sem_expr_region_data = sem_expr_region.data(db);
        let region_path = sem_expr_region_data.region_path();
        fn t<V>(len: usize) -> Vec<V> {
            let mut v: Vec<V> = vec![];
            v.reserve(len);
            v
        }
        SemaExprRangeCalculator {
            syn_expr_region_data,
            sem_expr_region_data,
            principal_entity_path_expr_ranges: t(syn_expr_region_data
                .principal_item_path_expr_arena()
                .len()),
            pattern_expr_ranges: t(syn_expr_region_data.pattern_expr_arena().len()),
            expr_ranges: t(sem_expr_region_data.sem_expr_arena().len()),
            stmt_ranges: SemaStmtMap::new(sem_expr_region_data.sem_stmt_arena()),
        }
    }

    fn calc_all(mut self) -> SemaExprRangeRegionData {
        // order matters
        for principal_entity_path_expr in self
            .syn_expr_region_data
            .principal_item_path_expr_arena()
            .iter()
        {
            self.principal_entity_path_expr_ranges
                .push(self.calc_principal_entity_path_expr_range(principal_entity_path_expr))
        }
        for pattern_expr in self.syn_expr_region_data.pattern_expr_arena().iter() {
            self.pattern_expr_ranges
                .push(self.calc_pattern_expr_range(pattern_expr))
        }
        for sem_expr_entry in self.sem_expr_region_data.sem_expr_arena().iter() {
            let expr_range = self.calc_expr_range(sem_expr_entry.data());
            self.expr_ranges.push(expr_range)
        }
        assert_eq!(
            self.sem_expr_region_data.sem_expr_arena().len(),
            self.expr_ranges.len()
        );
        SemaExprRangeRegionData {
            principal_entity_path_expr_ranges: self.principal_entity_path_expr_ranges,
            pattern_expr_ranges: self.pattern_expr_ranges,
            expr_ranges: self.expr_ranges,
            stmt_ranges: self.stmt_ranges,
        }
    }

    fn calc_principal_entity_path_expr_range(
        &self,
        expr: &SynPrincipalEntityPathExpr,
    ) -> RegionalTokenIdxRange {
        match expr {
            SynPrincipalEntityPathExpr::Root {
                path_name_token,
                principal_entity_path: item_path,
            } => RegionalTokenIdxRange::new_single(path_name_token.regional_token_idx()),
            SynPrincipalEntityPathExpr::Subitem {
                parent,
                colon_colon_token,
                ident_token,
                path,
            } => match ident_token {
                Ok(ident_token) => self[parent].to(RegionalTokenIdxRangeEnd::new_after(
                    ident_token.regional_token_idx(),
                )),
                Err(_) => self[parent].to(RegionalTokenIdxRangeEnd::new_after(
                    colon_colon_token.regional_token_idx(),
                )),
            },
        }
    }

    fn calc_pattern_expr_range(&self, expr: &SynPatternData) -> RegionalTokenIdxRange {
        match expr {
            SynPatternData::Literal {
                regional_token_idx, ..
            } => RegionalTokenIdxRange::new_single(*regional_token_idx),
            SynPatternData::Ident {
                symbol_modifier_tokens,
                ident_token,
            } => match symbol_modifier_tokens {
                Some(EphemSymbolModifierRegionalTokens::Owned(owned_token)) => {
                    RegionalTokenIdxRange::new_closed(
                        owned_token.regional_token_idx(),
                        ident_token.regional_token_idx(),
                    )
                }
                Some(EphemSymbolModifierRegionalTokens::Mut(mut_token)) => {
                    RegionalTokenIdxRange::new_closed(
                        mut_token.regional_token_idx(),
                        ident_token.regional_token_idx(),
                    )
                }
                Some(EphemSymbolModifierRegionalTokens::RefMut(ref_token, ..)) => {
                    RegionalTokenIdxRange::new_closed(
                        ref_token.regional_token_idx(),
                        ident_token.regional_token_idx(),
                    )
                }
                Some(_) => todo!(),
                None => RegionalTokenIdxRange::new_single(ident_token.regional_token_idx()),
            },
            SynPatternData::UnitTypeVariant { path_expr_idx, .. } => {
                self.principal_entity_path_expr_ranges[path_expr_idx.index()]
            }
            SynPatternData::Tuple { .. } => todo!(),
            SynPatternData::TupleStruct { .. } => todo!(),
            SynPatternData::TupleTypeVariant {
                path_expr_idx,
                rpar,
                ..
            } => self.principal_entity_path_expr_ranges[path_expr_idx.index()].to(
                RegionalTokenIdxRangeEnd::new_after(rpar.regional_token_idx()),
            ),
            SynPatternData::TupleStruct { .. } => todo!(),
            SynPatternData::TupleTypeVariant { .. } => todo!(),
            SynPatternData::Props { name, ref fields } => todo!(),
            SynPatternData::OneOf { options } => {
                let fst = options.elements().first().unwrap().syn_pattern();
                let lst = options.elements().last().unwrap().syn_pattern();
                let fst_range = self.pattern_expr_ranges[fst.index()];
                let lst_range = self.pattern_expr_ranges[lst.index()];
                fst_range.join(lst_range)
            }
            SynPatternData::Binding {
                ident_token,
                asperand_token,
                src,
            } => todo!(),
            SynPatternData::Range {
                start,
                dot_dot_token,
                end,
            } => todo!(),
        }
    }

    fn calc_expr_range(&mut self, expr: &SemaExprData) -> RegionalTokenIdxRange {
        match expr {
            SemaExprData::Literal(regional_token_idx, _)
            | SemaExprData::InheritedSynSymbol {
                regional_token_idx, ..
            }
            | SemaExprData::CurrentSynSymbol {
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
            SemaExprData::AssocItem {
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
                let end = self[target.syn_pattern_root().syn_pattern_expr_idx()].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemaExprData::Prefix {
                opr,
                opr_regional_token_idx,
                opd: opd_sem_expr_idx,
            } => RegionalTokenIdxRange::new(*opr_regional_token_idx, self[opd_sem_expr_idx].end()),
            SemaExprData::Suffix {
                opd: opd_sem_expr_idx,
                opr_regional_token_idx,
                ..
            }
            | SemaExprData::Unveil {
                opd_sem_expr_idx,
                opr_regional_token_idx,
                ..
            }
            | SemaExprData::Unwrap {
                opd_sem_expr_idx,
                opr_regional_token_idx,
                ..
            } => self[opd_sem_expr_idx]
                .to(RegionalTokenIdxRangeEnd::new_after(*opr_regional_token_idx)),
            SemaExprData::FunctionApplication {
                function_sem_expr_idx,
                argument_sem_expr_idx,
            } => self[function_sem_expr_idx].join(self[argument_sem_expr_idx]),
            SemaExprData::FunctionRitchieCall {
                function_sem_expr_idx,
                rpar_regional_token_idx,
                ..
            } => self[function_sem_expr_idx].to(RegionalTokenIdxRangeEnd::new_after(
                *rpar_regional_token_idx,
            )),
            SemaExprData::MethodApplication {
                self_argument: first_expr,
                rpar_regional_token_idx,
                ..
            } => todo!(),
            SemaExprData::MethodFnCall {
                self_argument_sem_expr_idx,
                rpar_regional_token_idx,
                ..
            } => self[self_argument_sem_expr_idx].to(RegionalTokenIdxRangeEnd::new_after(
                *rpar_regional_token_idx,
            )),
            SemaExprData::MethodGnCall { .. } => todo!(),
            SemaExprData::Field {
                self_argument: owner_sem_expr_idx,
                ident_token,
                ..
            } => self[owner_sem_expr_idx].to(RegionalTokenIdxRangeEnd::new_after(
                ident_token.regional_token_idx(),
            )),
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
            | SemaExprData::Delimitered {
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
            SemaExprData::Index {
                owner: owner_sem_expr_idx,
                lbox_regional_token_idx,
                rbox_regional_token_idx,
                ..
            } => self[owner_sem_expr_idx].to(RegionalTokenIdxRangeEnd::new_after(
                *rbox_regional_token_idx,
            )),
            SemaExprData::CompositionWithList {
                owner,
                lbox_regional_token_idx,
                rbox_regional_token_idx,
                ..
            } => self[owner].to(RegionalTokenIdxRangeEnd::new_after(
                *rbox_regional_token_idx,
            )),
            SemaExprData::NewList {
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
                return_ty_sem_expr_idx: return_ty_expr,
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
            SemaExprData::TemplateInstantiation {
                template,
                template_arguments,
            } => todo!(),
            &SemaExprData::VecFunctor {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
            } => {
                RegionalTokenIdxRange::new_closed(lbox_regional_token_idx, lbox_regional_token_idx)
            }
            &SemaExprData::ArrayFunctor {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
                ..
            } => {
                RegionalTokenIdxRange::new_closed(lbox_regional_token_idx, lbox_regional_token_idx)
            }
            &SemaExprData::NestedBlock {
                lcurl_regional_token_idx,
                stmts,
                rcurl_regional_token,
            } => RegionalTokenIdxRange::new_closed(
                lcurl_regional_token_idx,
                rcurl_regional_token.regional_token_idx(),
            ),
            SemaExprData::VecFunctor {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
            } => todo!(),
            SemaExprData::ArrayFunctor {
                lbox_regional_token_idx,
                items,
                rbox_regional_token_idx,
            } => todo!(),
            &SemaExprData::Closure {
                closure_kind_regional_token_idx,
                lvert_regional_token_idx,
                body,
                ..
            } => {
                let start = closure_kind_regional_token_idx.unwrap_or(lvert_regional_token_idx);
                RegionalTokenIdxRange::new(start, self[body].end())
            }
            SemaExprData::NestedBlock {
                lcurl_regional_token_idx,
                stmts,
                rcurl_regional_token,
            } => todo!(),
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
        self.stmt_ranges.insert_new(stmt_idx, range)
    }

    fn calc_stmt_range(&mut self, stmt_idx: SemaStmtIdx) -> RegionalTokenIdxRange {
        match stmt_idx.data(self.sem_expr_region_data.sem_stmt_arena()) {
            SemaStmtData::Let {
                let_token,
                initial_value_sem_expr_idx,
                ..
            } => {
                let start = let_token.regional_token_idx();
                let end = self[initial_value_sem_expr_idx].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemaStmtData::Return {
                return_token,
                ref result,
                ..
            } => {
                let start = return_token.regional_token_idx();
                let end = self[result].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemaStmtData::Require {
                require_token,
                condition,
            } => {
                let start = require_token.regional_token_idx();
                let end = self.calc_condition_end(*condition);
                RegionalTokenIdxRange::new(start, end)
            }
            &SemaStmtData::Assert {
                assert_token,
                condition,
            } => {
                let start = assert_token.regional_token_idx();
                let end = self.calc_condition_end(condition);
                RegionalTokenIdxRange::new(start, end)
            }
            SemaStmtData::Break { break_token } => {
                RegionalTokenIdxRange::new_single(break_token.regional_token_idx())
            }
            SemaStmtData::Eval { sem_expr_idx, .. } => self[sem_expr_idx],
            SemaStmtData::ForBetween {
                for_token,
                ref particulars,
                ref eol_colon,
                stmts: ref block,
                ..
            } => {
                let start = for_token.regional_token_idx();
                let end = self.calc_block_range(*block).end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemaStmtData::ForIn {
                for_token,
                stmts: ref block,
                ..
            } => todo!(),
            SemaStmtData::Forext {
                forext_token,
                /* todo: particulars */
                ref eol_colon,
                stmts: ref block,
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
                stmts: ref block,
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
                stmts: ref block,
                ..
            } => {
                let start = do_token.regional_token_idx();
                let end = self.calc_block_range(*block).end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemaStmtData::IfElse {
                if_branch: ref sem_if_branch,
                elif_branches: ref sem_elif_branches,
                else_branch: ref sem_else_branch,
                ..
            } => {
                let start = sem_if_branch.if_token().regional_token_idx();
                // it's important that every branch is computed
                let if_branch_end: RegionalTokenIdxRangeEnd =
                    self.calc_block_range(sem_if_branch.stmts()).end();
                let mut elif_branch_rev_iter = sem_elif_branches.iter().rev();
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
                    if let Some(else_branch) = sem_else_branch {
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

    fn calc_condition_end(&self, condition: SemaCondition) -> RegionalTokenIdxRangeEnd {
        match condition {
            SemaCondition::Be { target, .. } => {
                self[target.syn_pattern_root().syn_pattern_expr_idx()].end()
            }
            SemaCondition::Other {
                sem_expr_idx,
                conversion,
            } => self[sem_expr_idx].end(),
        }
    }
}
