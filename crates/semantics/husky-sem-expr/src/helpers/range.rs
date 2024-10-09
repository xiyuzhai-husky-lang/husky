use self::condition::SemCondition;

use super::*;
use husky_print_utils::p;
use husky_regional_token::{
    EphemSymbolModifierRegionalTokens, RegionalTokenIdxRange, RegionalTokenIdxRangeEnd,
};
use husky_syn_expr::entity_path::{SynPrincipalEntityPathExpr, SynPrincipalEntityPathExprIdx};
use husky_vfs::path::module_path::ModulePath;

#[salsa::tracked(db = SemExprDb, jar = SemExprJar)]
pub struct SemExprRangeRegion {
    #[return_ref]
    pub data: SemExprRangeRegionData,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SemExprRangeRegionData {
    principal_entity_path_expr_ranges: Vec<RegionalTokenIdxRange>,
    pattern_ranges: Vec<RegionalTokenIdxRange>,
    expr_ranges: Vec<RegionalTokenIdxRange>,
    // use SemStmtMap because the inference order may not be in consistent with stmt id
    stmt_ranges: SemStmtMap<RegionalTokenIdxRange>,
}

#[salsa::tracked(jar = SemExprJar)]
pub fn sem_expr_range_region(
    db: &::salsa::Db,
    sem_expr_region: SemExprRegion,
) -> SemExprRangeRegion {
    SemExprRangeRegion::new(
        db,
        SemExprRangeCalculator::new(db, sem_expr_region).calc_all(),
    )
}

#[cfg(test)]
fn decl_sem_expr_range_regions(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> Vec<SemExprRangeRegion> {
    use husky_syn_decl::sheet::HasSynDeclSheet;

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
    DB::ast_rich_test_debug_with_db(
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
) -> Vec<SemExprRangeRegion> {
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
    DB::ast_rich_test_debug_with_db(
        defn_sem_expr_range_regions,
        &AstTestConfig::new(
            "defn_sem_expr_range_regions",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    );
}

impl std::ops::Index<SynPrincipalEntityPathExprIdx> for SemExprRangeRegionData {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynPrincipalEntityPathExprIdx) -> &Self::Output {
        &self.principal_entity_path_expr_ranges[index.index()]
    }
}

impl std::ops::Index<SynPatternIdx> for SemExprRangeRegionData {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynPatternIdx) -> &Self::Output {
        &self.pattern_ranges[index.index()]
    }
}

impl std::ops::Index<SemExprIdx> for SemExprRangeRegionData {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SemExprIdx) -> &Self::Output {
        &self.expr_ranges[index.index()]
    }
}

impl std::ops::Index<SemStmtIdx> for SemExprRangeRegionData {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SemStmtIdx) -> &Self::Output {
        &self.stmt_ranges[index]
    }
}

struct SemExprRangeCalculator<'a> {
    syn_expr_region_data: &'a SynExprRegionData,
    sem_expr_region_data: &'a SemExprRegionData,
    principal_entity_path_expr_ranges: Vec<RegionalTokenIdxRange>,
    pattern_ranges: Vec<RegionalTokenIdxRange>,
    expr_ranges: Vec<RegionalTokenIdxRange>,
    stmt_ranges: SemStmtMap<RegionalTokenIdxRange>,
}

impl<'a> std::ops::Index<SynPrincipalEntityPathExprIdx> for SemExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynPrincipalEntityPathExprIdx) -> &Self::Output {
        &self.principal_entity_path_expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<&SynPrincipalEntityPathExprIdx> for SemExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: &SynPrincipalEntityPathExprIdx) -> &Self::Output {
        &self.principal_entity_path_expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<SynPatternIdx> for SemExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SynPatternIdx) -> &Self::Output {
        &self.pattern_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<SemExprIdx> for SemExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SemExprIdx) -> &Self::Output {
        &self.expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<&SemExprIdx> for SemExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: &SemExprIdx) -> &Self::Output {
        &self.expr_ranges[index.index()]
    }
}

impl<'a> std::ops::Index<SemStmtIdx> for SemExprRangeCalculator<'a> {
    type Output = RegionalTokenIdxRange;

    fn index(&self, index: SemStmtIdx) -> &Self::Output {
        &self.stmt_ranges[index]
    }
}

impl<'a> SemExprRangeCalculator<'a> {
    fn new(db: &'a ::salsa::Db, sem_expr_region: SemExprRegion) -> Self {
        let syn_expr_region = sem_expr_region.syn_expr_region(db);
        let syn_expr_region_data = syn_expr_region.data(db);
        let sem_expr_region_data = sem_expr_region.data(db);
        let region_path = sem_expr_region_data.region_path();
        fn t<V>(len: usize) -> Vec<V> {
            let mut v: Vec<V> = vec![];
            v.reserve(len);
            v
        }
        SemExprRangeCalculator {
            syn_expr_region_data,
            sem_expr_region_data,
            principal_entity_path_expr_ranges: t(syn_expr_region_data
                .principal_item_path_expr_arena()
                .len()),
            pattern_ranges: t(syn_expr_region_data.pattern_arena().len()),
            expr_ranges: t(sem_expr_region_data.sem_expr_arena().len()),
            stmt_ranges: SemStmtMap::new(sem_expr_region_data.sem_stmt_arena()),
        }
    }

    fn calc_all(mut self) -> SemExprRangeRegionData {
        // order matters
        for principal_entity_path_expr in self
            .syn_expr_region_data
            .principal_item_path_expr_arena()
            .iter()
        {
            self.principal_entity_path_expr_ranges
                .push(self.calc_principal_entity_path_expr_range(principal_entity_path_expr))
        }
        for pattern in self.syn_expr_region_data.pattern_arena().iter() {
            self.pattern_ranges.push(self.calc_pattern_range(pattern))
        }
        for sem_expr_entry in self.sem_expr_region_data.sem_expr_arena().iter() {
            let expr_range = self.calc_expr_range(sem_expr_entry.data());
            self.expr_ranges.push(expr_range)
        }
        assert_eq!(
            self.sem_expr_region_data.sem_expr_arena().len(),
            self.expr_ranges.len()
        );
        SemExprRangeRegionData {
            principal_entity_path_expr_ranges: self.principal_entity_path_expr_ranges,
            pattern_ranges: self.pattern_ranges,
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

    fn calc_pattern_range(&self, expr: &SynPatternData) -> RegionalTokenIdxRange {
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
                let fst_range = self.pattern_ranges[fst.index()];
                let lst_range = self.pattern_ranges[lst.index()];
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

    fn calc_expr_range(&mut self, expr: &SemExprData) -> RegionalTokenIdxRange {
        match expr {
            SemExprData::Literal(regional_token_idx, _)
            | SemExprData::InheritedVariable {
                regional_token_idx, ..
            }
            | SemExprData::CurrentVariable {
                regional_token_idx, ..
            }
            | SemExprData::FrameVarDecl {
                regional_token_idx, ..
            }
            | SemExprData::SelfType(regional_token_idx)
            | SemExprData::SelfValue(regional_token_idx) => {
                RegionalTokenIdxRange::new_single(*regional_token_idx)
            }
            SemExprData::Binary { lopd, ropd, .. } => self[lopd].join(self[ropd]),
            SemExprData::PrincipalEntityPath { path_expr_idx, .. } => self[*path_expr_idx],
            SemExprData::MajorItemPathAssocItem {
                parent_expr_idx,
                ident_token,
                ..
            } => {
                // todo: consider implicit(angular) arguments
                self[parent_expr_idx].to(RegionalTokenIdxRangeEnd::new_after(
                    ident_token.regional_token_idx(),
                ))
            }
            &SemExprData::TypeAsTraitItem {
                lpar_regional_token_idx,
                ident_regional_token_idx,
                ..
            } => {
                RegionalTokenIdxRange::new_closed(lpar_regional_token_idx, ident_regional_token_idx)
            }
            &SemExprData::AssocItem {
                parent_expr_idx,
                ident_regional_token_idx,
                ..
            } => self[parent_expr_idx].to(RegionalTokenIdxRangeEnd::new_after(
                ident_regional_token_idx,
            )),
            SemExprData::Be {
                src,
                contract,
                be_regional_token_idx,
                target,
            } => {
                let start = self[src].start().regional_token_idx();
                let end = self[target.syn_pattern_root().syn_pattern_idx()].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemExprData::Prefix {
                opr,
                opr_regional_token_idx,
                opd: opd_sem_expr_idx,
            } => RegionalTokenIdxRange::new(*opr_regional_token_idx, self[opd_sem_expr_idx].end()),
            SemExprData::Suffix {
                opd: opd_sem_expr_idx,
                opr_regional_token_idx,
                ..
            }
            | SemExprData::Unveil {
                opd: opd_sem_expr_idx,
                opr_regional_token_idx,
                ..
            }
            | SemExprData::Unwrap {
                opd: opd_sem_expr_idx,
                opr_regional_token_idx,
                ..
            } => self[opd_sem_expr_idx]
                .to(RegionalTokenIdxRangeEnd::new_after(*opr_regional_token_idx)),
            SemExprData::FunctionApplication {
                function: function_sem_expr_idx,
                argument: argument_sem_expr_idx,
            } => self[function_sem_expr_idx].join(self[argument_sem_expr_idx]),
            SemExprData::FunctionRitchieCall {
                function: function_sem_expr_idx,
                rpar_regional_token_idx,
                ..
            } => self[function_sem_expr_idx].to(RegionalTokenIdxRangeEnd::new_after(
                *rpar_regional_token_idx,
            )),
            SemExprData::MethodApplication {
                self_argument: first_expr,
                rpar_regional_token_idx,
                ..
            } => todo!(),
            SemExprData::MethodRitchieCall {
                self_argument: self_argument_sem_expr_idx,
                rpar_regional_token_idx,
                ..
            } => self[self_argument_sem_expr_idx].to(RegionalTokenIdxRangeEnd::new_after(
                *rpar_regional_token_idx,
            )),
            SemExprData::Field {
                self_argument: owner_sem_expr_idx,
                ident_token,
                ..
            } => self[owner_sem_expr_idx].to(RegionalTokenIdxRangeEnd::new_after(
                ident_token.regional_token_idx(),
            )),
            SemExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => match place_label_regional_token {
                Some(_) => todo!(),
                None => RegionalTokenIdxRange::new_single(*at_regional_token_idx),
            },
            SemExprData::Unit {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
                ..
            }
            | SemExprData::Delimitered {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
                ..
            }
            | SemExprData::NewTuple {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
                ..
            } => RegionalTokenIdxRange::new(
                *lpar_regional_token_idx,
                RegionalTokenIdxRangeEnd::new_after(*rpar_regional_token_idx),
            ),
            SemExprData::Index {
                self_argument: owner_sem_expr_idx,
                lbox_regional_token_idx,
                rbox_regional_token_idx,
                ..
            } => self[owner_sem_expr_idx].to(RegionalTokenIdxRangeEnd::new_after(
                *rbox_regional_token_idx,
            )),
            SemExprData::CompositionWithList {
                owner,
                lbox_regional_token_idx,
                rbox_regional_token_idx,
                ..
            } => self[owner].to(RegionalTokenIdxRangeEnd::new_after(
                *rbox_regional_token_idx,
            )),
            SemExprData::NewList {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
                ..
            } => RegionalTokenIdxRange::new(
                *lbox_regional_token_idx,
                RegionalTokenIdxRangeEnd::new_after(*rbox_regional_token_idx),
            ),
            SemExprData::BoxColonList {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
                ..
            } => RegionalTokenIdxRange::new(
                *lbox_regional_token_idx,
                RegionalTokenIdxRangeEnd::new_after(*rbox_regional_token_idx),
            ),
            SemExprData::Block { stmts } => self.calc_block_range(*stmts),
            &SemExprData::NestedBlock {
                lcurl_regional_token_idx,
                stmts,
                rcurl_regional_token,
            } => {
                let _ = self.calc_block_range(stmts);
                RegionalTokenIdxRange::new_closed(
                    lcurl_regional_token_idx,
                    rcurl_regional_token.regional_token_idx(),
                )
            }
            SemExprData::EmptyHtmxTag {
                empty_htmx_bra_idx: empty_html_bra_idx,
                empty_htmx_ket: empty_html_ket,
                ..
            } => RegionalTokenIdxRange::new_closed(
                *empty_html_bra_idx,
                empty_html_ket.regional_token_idx(),
            ),
            SemExprData::Ritchie {
                ritchie_kind_regional_token_idx,
                rpar_regional_token_idx,
                return_ty: return_ty_expr,
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
            SemExprData::Sorry { regional_token_idx }
            | SemExprData::Todo { regional_token_idx }
            | SemExprData::Unreachable { regional_token_idx } => {
                RegionalTokenIdxRange::new_single(*regional_token_idx)
            }
            SemExprData::TemplateInstantiation {
                template,
                template_arguments,
            } => todo!(),
            &SemExprData::VecFunctor {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
            } => {
                RegionalTokenIdxRange::new_closed(lbox_regional_token_idx, lbox_regional_token_idx)
            }
            &SemExprData::ArrayFunctor {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
                ..
            } => {
                RegionalTokenIdxRange::new_closed(lbox_regional_token_idx, lbox_regional_token_idx)
            }
            SemExprData::VecFunctor {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
            } => todo!(),
            SemExprData::ArrayFunctor {
                lbox_regional_token_idx,
                items,
                rbox_regional_token_idx,
            } => todo!(),
            &SemExprData::Closure {
                closure_kind_regional_token_idx,
                lvert_regional_token_idx,
                body,
                ..
            } => {
                let start = closure_kind_regional_token_idx.unwrap_or(lvert_regional_token_idx);
                RegionalTokenIdxRange::new(start, self[body].end())
            }
            SemExprData::NestedBlock {
                lcurl_regional_token_idx,
                stmts,
                rcurl_regional_token,
            } => todo!(),
        }
    }

    fn calc_block_range(&mut self, stmts: SemStmtIdxRange) -> RegionalTokenIdxRange {
        for stmt in stmts {
            self.save_stmt_range(stmt);
        }
        self[stmts.start()].join(self[stmts.end() - 1])
    }

    fn save_stmt_range(&mut self, stmt_idx: SemStmtIdx) {
        let range = self.calc_stmt_range(stmt_idx);
        self.stmt_ranges.insert_new(stmt_idx, range)
    }

    fn calc_stmt_range(&mut self, stmt_idx: SemStmtIdx) -> RegionalTokenIdxRange {
        match stmt_idx.data(self.sem_expr_region_data.sem_stmt_arena()) {
            SemStmtData::Let {
                let_token,
                initial_value: initial_value_sem_expr_idx,
                ..
            } => {
                let start = let_token.regional_token_idx();
                let end = self[initial_value_sem_expr_idx].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemStmtData::Return {
                return_token,
                ref result,
                ..
            } => {
                let start = return_token.regional_token_idx();
                let end = self[result].end();
                RegionalTokenIdxRange::new(start, end)
            }
            SemStmtData::Require {
                require_token,
                condition,
            } => {
                let start = require_token.regional_token_idx();
                let end = self.calc_condition_end(*condition);
                RegionalTokenIdxRange::new(start, end)
            }
            &SemStmtData::Assert {
                assert_token,
                condition,
            } => {
                let start = assert_token.regional_token_idx();
                let end = self.calc_condition_end(condition);
                RegionalTokenIdxRange::new(start, end)
            }
            SemStmtData::Break { break_token } => {
                RegionalTokenIdxRange::new_single(break_token.regional_token_idx())
            }
            SemStmtData::Eval {
                expr: sem_expr_idx, ..
            } => self[sem_expr_idx],
            SemStmtData::ForBetween {
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
            SemStmtData::ForIn {
                for_token,
                stmts: ref block,
                ..
            } => todo!(),
            SemStmtData::Forext {
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
            SemStmtData::While {
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
            SemStmtData::DoWhile {
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
            SemStmtData::IfElse {
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
            SemStmtData::Match {
                match_token,
                case_branches,
                eol_with_token,
                ..
            } => {
                let mut case_branch_rev_iter = case_branches.iter().rev();
                let last_case_branch_range_end =
                    if let Some(last_case_branch) = case_branch_rev_iter.next() {
                        Some(self.calc_block_range(last_case_branch.stmts).end())
                    } else {
                        None
                    };
                for case_branch in case_branch_rev_iter {
                    let _ = self.calc_block_range(case_branch.stmts);
                }
                match last_case_branch_range_end {
                    Some(last_case_branch_range_end) => RegionalTokenIdxRange::new(
                        match_token.regional_token_idx(),
                        last_case_branch_range_end,
                    ),
                    _ => RegionalTokenIdxRange::new_closed(
                        match_token.regional_token_idx(),
                        eol_with_token.regional_token_idx(),
                    ),
                }
            }
            SemStmtData::Assert {
                assert_token,
                condition,
            } => todo!(),
            SemStmtData::Narrate { narrate_token } => {
                // ad hoc
                RegionalTokenIdxRange::new_single(narrate_token.regional_token_idx())
            }
        }
    }

    fn calc_condition_end(&self, condition: SemCondition) -> RegionalTokenIdxRangeEnd {
        match condition {
            SemCondition::Be { target, .. } => {
                self[target.syn_pattern_root().syn_pattern_idx()].end()
            }
            SemCondition::Other {
                expr: sem_expr_idx,
                conversion,
            } => self[sem_expr_idx].end(),
        }
    }
}
