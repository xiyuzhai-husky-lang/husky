use super::region::sem_expr_region_from_region_path;
use super::*;
use crate::{SemExprData, SemExprIdx, SemExprRegionData, SemStmtIdx, SemStmtIdxRange};
use closure_parameter::ClosureParameterObelisk;
use condition::SemCondition;
use husky_entity_path::region::RegionPath;
use husky_entity_tree::{
    helpers::tokra_region::HasRegionalTokenIdxBase, node::ItemSynNodePath,
    region_path::SynNodeRegionPath,
};
use husky_regional_token::RegionalTokenIdxBase;
use husky_text::{HasText, Text};
use husky_token::RangedTokenSheet;
use if_else_stmt::{SemElifBranch, SemElseBranch, SemIfBranch};
use range::{sem_expr_range_region, SemExprRangeRegionData};

pub trait VisitSemExpr<'db>: Sized {
    fn db(&self) -> &'db ::salsa::Db;
    fn sem_expr_region_data(&self) -> &'db SemExprRegionData;
    fn visit_expr(&mut self, expr: SemExprIdx, f: impl FnOnce(&mut Self));
    fn visit_expr_itself(&mut self, expr: SemExprIdx);
    fn visit_closure_inner(
        &mut self,
        expr: SemExprIdx,
        parameters: &[ClosureParameterObelisk],
        f: impl FnOnce(&mut Self),
    );
    fn visit_stmts(&mut self, stmts: SemStmtIdxRange, f: impl FnOnce(&mut Self));
    /// `f` is a closure in which
    /// - first the subexpression and substatements of `stmt` will be simulated in evaluation order
    /// - then `visit_stmt_inner` will be invoked
    ///
    /// this method `visit_stmt` should set up the proper environment and invoke `f`
    fn visit_stmt(&mut self, stmt: SemStmtIdx, f: impl FnOnce(&mut Self));
    fn visit_stmt_itself(&mut self, stmt: SemStmtIdx);
    /// `f` is a closure that can be called for arbitrary times, in which
    /// - the precondition is simulated if it exists
    /// - the loop body is simulated
    /// - the postcondition is simulated if it exists
    fn visit_for_loop_stmt_inner(
        &mut self,
        stmt: SemStmtIdx,
        for_loop_varible_idx: CurrentVariableIdx,
        f: impl Fn(&mut Self),
    );
    fn visit_loop(&mut self, stmt: SemStmtIdx, f: impl Fn(&mut Self));
    fn visit_branches(&mut self, f: impl Fn(&mut Self));
    fn visit_branch(&mut self, f: impl Fn(&mut Self));
    fn visit_condition(&mut self, condition: SemCondition, f: impl FnOnce(&mut Self));
    fn visit_condition_inner(&mut self, condition: SemCondition);
    /// final
    fn visit_all(&mut self, f: impl Fn(SynExprRootKind) -> bool) {
        for (expr, root_kind) in self.sem_expr_region_data().sem_expr_roots() {
            if f(root_kind) {
                expr.simulate(self)
            }
        }
    }
    /// final
    fn visit_root_body(&mut self) {
        let root_body = self.sem_expr_region_data().root_body();
        root_body.simulate(self)
    }
}

impl SemExprIdx {
    fn simulate<'db>(self, visitor: &mut impl VisitSemExpr<'db>) {
        visitor.visit_expr(self, |visitor| {
            let sem_expr_region_data = visitor.sem_expr_region_data();
            let sem_expr_arena_ref = sem_expr_region_data.sem_expr_arena();
            match *self.data(sem_expr_arena_ref) {
                SemExprData::Literal(_, _)
                | SemExprData::Unit { .. }
                | SemExprData::PrincipalEntityPath { .. }
                | SemExprData::MajorItemPathAssocItem { .. }
                | SemExprData::AssocItem { .. }
                | SemExprData::InheritedVariable { .. }
                | SemExprData::CurrentVariable { .. }
                | SemExprData::FrameVarDecl { .. }
                | SemExprData::SelfType(_)
                | SemExprData::SelfValue(_) => (),
                SemExprData::TypeAsTraitItem { ty, trai, .. } => {
                    ty.simulate(visitor);
                    trai.simulate(visitor);
                }
                SemExprData::Binary { lopd, ropd, .. } => {
                    lopd.simulate(visitor);
                    ropd.simulate(visitor);
                }
                SemExprData::Be {
                    src,
                    contract,
                    be_regional_token_idx,
                    target,
                } => todo!(),
                SemExprData::Prefix { opd, .. }
                | SemExprData::Suffix { opd, .. }
                | SemExprData::Unveil { opd, .. }
                | SemExprData::Unwrap { opd, .. } => {
                    opd.simulate(visitor);
                }
                SemExprData::FunctionApplication { function, argument } => {
                    function.simulate(visitor);
                    argument.simulate(visitor);
                }
                SemExprData::FunctionRitchieCall {
                    function,
                    ref template_arguments,
                    ref ritchie_parameter_argument_matches,
                    ..
                } => {
                    function.simulate(visitor);
                    if let Some(_) = template_arguments {
                        todo!()
                    }
                    simulate_ritchie_parameter_argument_matches(
                        ritchie_parameter_argument_matches,
                        visitor,
                    );
                }
                SemExprData::Ritchie {
                    ref parameter_ty_items,
                    return_ty,
                    ..
                } => {
                    for item in parameter_ty_items {
                        item.sem_expr_idx.simulate(visitor);
                    }
                    if let Some(return_ty) = return_ty {
                        return_ty.simulate(visitor);
                    }
                }
                SemExprData::Field { self_argument, .. } => self_argument.simulate(visitor),
                SemExprData::MethodApplication {
                    self_argument,
                    ref template_arguments,
                    ref items,
                    ..
                } => todo!(),
                SemExprData::MethodRitchieCall {
                    self_argument,
                    ref dispatch,
                    ref template_arguments,
                    ref ritchie_parameter_argument_matches,
                    ..
                } => {
                    self_argument.simulate(visitor);
                    if let Some(template_arguments) = template_arguments {
                        todo!()
                    }
                    simulate_ritchie_parameter_argument_matches(
                        ritchie_parameter_argument_matches,
                        visitor,
                    );
                }
                SemExprData::TemplateInstantiation {
                    template,
                    ref template_arguments,
                } => todo!(),
                SemExprData::At { .. } => (),
                SemExprData::Delimitered { item, .. } => item.simulate(visitor),
                SemExprData::NewTuple {
                    lpar_regional_token_idx,
                    ref items,
                    rpar_regional_token_idx,
                } => todo!(),
                SemExprData::Index {
                    owner,
                    ref index_sem_list_items,
                    ..
                } => {
                    owner.simulate(visitor);
                    simulate_comma_list_items(index_sem_list_items, visitor);
                }
                SemExprData::CompositionWithList {
                    owner, ref items, ..
                } => todo!(),
                SemExprData::NewList { ref items, .. } => simulate_comma_list_items(items, visitor),
                SemExprData::BoxColonList { ref items, .. } => {
                    simulate_comma_list_items(items, visitor);
                }
                SemExprData::VecFunctor { .. } => (),
                SemExprData::ArrayFunctor { ref items, .. } => {
                    for item in items {
                        item.sem_expr_idx.simulate(visitor);
                    }
                }
                SemExprData::Block { stmts } => stmts.simulate(visitor),
                SemExprData::EmptyHtmlTag {
                    empty_html_bra_idx,
                    function_ident,
                    ref arguments,
                    empty_html_ket,
                } => {
                    // ad hoc
                    for argument in arguments {
                        match argument {
                            SemaHtmlArgumentExpr::Expanded {
                                property_ident,
                                eq,
                                lcurl,
                                argument,
                                rcurl,
                            } => argument.simulate(visitor),
                            SemaHtmlArgumentExpr::Shortened {
                                lcurl,
                                property_ident,
                                rcurl,
                            } => (), // todo: argument: SemExprIdx
                        }
                    }
                }
                // ad hoc
                SemExprData::Closure {
                    closure_kind_regional_token_idx,
                    lvert_regional_token_idx,
                    ref parameter_obelisks,
                    rvert_regional_token,
                    return_ty,
                    body,
                } => {
                    visitor.visit_closure_inner(self, parameter_obelisks, |visitor| {
                        body.simulate(visitor)
                    });
                }
                SemExprData::Sorry { regional_token_idx } => todo!(),
                // ad hoc
                SemExprData::Todo { .. } => (),
                // ad hoc
                SemExprData::Unreachable { .. } => (),
                SemExprData::NestedBlock { stmts, .. } => stmts.simulate(visitor),
            }
            visitor.visit_expr_itself(self)
        });
    }
}

fn simulate_comma_list_items<'db>(
    items: &[SemaCommaListItem],
    visitor: &mut impl VisitSemExpr<'db>,
) {
    for item in items {
        item.sem_expr_idx.simulate(visitor);
    }
}

fn simulate_ritchie_parameter_argument_matches<'db>(
    ritchie_parameter_argument_matches: &SmallVec<[SemaRitchieArgument; 4]>,
    visitor: &mut impl VisitSemExpr<'db>,
) {
    for m in ritchie_parameter_argument_matches {
        match m {
            SemaRitchieArgument::Simple(_, arg) => {
                arg.argument_sem_expr_idx().simulate(visitor);
            }
            SemaRitchieArgument::Variadic(_, items) => {
                for item in items {
                    item.argument_expr_idx().simulate(visitor);
                }
            }
            SemaRitchieArgument::Keyed(_, arg) => match arg {
                Some(arg) => arg.argument_expr_idx().simulate(visitor),
                // ad hoc
                None => (),
            },
        }
    }
}

impl SemStmtIdxRange {
    fn simulate<'db>(self, visitor: &mut impl VisitSemExpr<'db>) {
        visitor.visit_stmts(self, |visitor| {
            for stmt in self {
                stmt.simulate(visitor)
            }
        })
    }
}

impl SemStmtIdx {
    fn simulate<'db>(self, visitor: &mut impl VisitSemExpr<'db>) {
        visitor.visit_stmt(self, |visitor| {
            let sem_expr_region_data = visitor.sem_expr_region_data();
            let sem_stmt_arena_ref = sem_expr_region_data.sem_stmt_arena();
            match *self.data(sem_stmt_arena_ref) {
                SemStmtData::Let { initial_value, .. } => initial_value.simulate(visitor),
                SemStmtData::Return { result, .. } => result.simulate(visitor),
                SemStmtData::Require { condition, .. } | SemStmtData::Assert { condition, .. } => {
                    condition.simulate(visitor)
                }
                SemStmtData::Break { .. } => (),
                SemStmtData::Eval { expr, .. } => expr.simulate(visitor),
                SemStmtData::ForBetween {
                    for_token,
                    ref particulars,
                    for_loop_varible_idx,
                    eol_colon,
                    stmts,
                } => {
                    let range = particulars.range();
                    range
                        .initial_boundary
                        .bound_expr
                        .map(|bound_expr| bound_expr.simulate(visitor));
                    range
                        .final_boundary
                        .bound_expr
                        .map(|bound_expr| bound_expr.simulate(visitor));
                    visitor.visit_for_loop_stmt_inner(self, for_loop_varible_idx, |visitor| {
                        visitor.visit_loop(self, |visitor| stmts.simulate(visitor))
                    })
                }
                SemStmtData::ForIn {
                    for_token,
                    range,
                    eol_colon,
                    stmts,
                } => todo!(),
                SemStmtData::Forext {
                    forext_token,
                    ref particulars,
                    eol_colon,
                    stmts,
                } => {
                    // ad hoc
                    particulars.bound_expr.simulate(visitor);
                    stmts.simulate(visitor);
                }
                SemStmtData::While {
                    while_token,
                    condition,
                    eol_colon,
                    stmts,
                } => {
                    visitor.visit_loop(self, |visitor| {
                        condition.simulate(visitor);
                        stmts.simulate(visitor);
                    });
                }
                SemStmtData::DoWhile {
                    do_token,
                    while_token,
                    condition,
                    eol_colon,
                    stmts,
                } => {
                    visitor.visit_loop(self, |visitor| {
                        stmts.simulate(visitor);
                        condition.simulate(visitor);
                    });
                }
                SemStmtData::IfElse {
                    ref if_branch,
                    ref elif_branches,
                    ref else_branch,
                } => visitor.visit_branches(|visitor| {
                    visitor.visit_branch(|visitor| if_branch.simulate(visitor));
                    for elif_branch in elif_branches {
                        visitor.visit_branch(|visitor| elif_branch.simulate(visitor));
                    }
                    if let Some(else_branch) = else_branch {
                        visitor.visit_branch(|visitor| else_branch.simulate(visitor));
                    }
                }),
                SemStmtData::Match {
                    match_token,
                    opd: match_opd,
                    contract: match_contract,
                    eol_with_token,
                    ref case_branches,
                } => {
                    match_opd.simulate(visitor);
                    visitor.visit_branches(|visitor| {
                        for case_branch in case_branches {
                            visitor.visit_branch(|visitor| case_branch.stmts.simulate(visitor));
                        }
                    })
                }
                // ad hoc
                SemStmtData::Narrate { .. } => (),
            };
            visitor.visit_stmt_itself(self);
        })
    }
}

impl SemCondition {
    fn simulate<'db>(self, visitor: &mut impl VisitSemExpr<'db>) {
        visitor.visit_condition(self, |visitor| {
            match self {
                SemCondition::Be { src, .. } => src.simulate(visitor),
                SemCondition::Other { sem_expr_idx, .. } => sem_expr_idx.simulate(visitor),
            };
            visitor.visit_condition_inner(self)
        })
    }
}

impl SemIfBranch {
    fn simulate<'db>(&self, visitor: &mut impl VisitSemExpr<'db>) {
        // ad hoc
        self.condition.simulate(visitor);
        self.stmts.simulate(visitor);
    }
}

impl SemElifBranch {
    fn simulate<'db>(&self, visitor: &mut impl VisitSemExpr<'db>) {
        // ad hoc
        self.condition.simulate(visitor);
        self.stmts.simulate(visitor);
    }
}

impl SemElseBranch {
    fn simulate<'db>(&self, visitor: &mut impl VisitSemExpr<'db>) {
        // ad hoc
        self.stmts.simulate(visitor);
    }
}

#[test]
fn visit_sem_expr_works() {
    struct SemExprVisitor<'db> {
        db: &'db ::salsa::Db,
        text: Text<'db>,
        sem_expr_region_data: &'db SemExprRegionData,
        sem_expr_range_region_data: &'db SemExprRangeRegionData,
        syn_expr_range_region: &'db SynExprRangeRegion,
        ranged_token_sheet: &'db RangedTokenSheet,
        base: Option<RegionalTokenIdxBase>,
        visits: Vec<&'db str>,
    }

    impl<'db> SemExprVisitor<'db> {
        fn new(region_path: RegionPath, db: &'db ::salsa::Db) -> Option<Self> {
            use husky_token::TokenDb;
            let module_path = region_path.module_path(db);
            let text = module_path.text(db);
            let sem_expr_region = sem_expr_region_from_region_path(region_path, db)?;
            let sem_expr_range_region_data = sem_expr_range_region(db, sem_expr_region).data(db);
            let sem_expr_region_data = sem_expr_region.data(db);
            let syn_expr_region = sem_expr_region.syn_expr_region(db);
            let syn_expr_range_region = syn_expr_region.range_region(db);
            let ranged_token_sheet = db.ranged_token_sheet(module_path);
            Some(Self {
                db,
                text,
                ranged_token_sheet,
                sem_expr_region_data,
                sem_expr_range_region_data,
                syn_expr_range_region,
                base: sem_expr_region_data.path().regional_token_idx_base(db),
                visits: vec![],
            })
        }
    }

    impl<'db> VisitSemExpr<'db> for SemExprVisitor<'db> {
        fn db(&self) -> &'db salsa::Db {
            self.db
        }

        fn sem_expr_region_data(&self) -> &'db SemExprRegionData {
            self.sem_expr_region_data
        }

        fn visit_expr(&mut self, expr: SemExprIdx, f: impl FnOnce(&mut Self)) {
            f(self)
        }

        fn visit_expr_itself(&mut self, expr: SemExprIdx) {
            let token_idx_range =
                self.sem_expr_range_region_data[expr].token_idx_range(self.base.unwrap());
            let text_range = self.ranged_token_sheet.tokens_text_range(token_idx_range);
            self.visits.push(self.text.text_within(text_range));
        }

        fn visit_closure_inner(
            &mut self,
            expr: SemExprIdx,
            parameters: &[ClosureParameterObelisk],
            f: impl FnOnce(&mut Self),
        ) {
            f(self)
        }

        fn visit_stmts(&mut self, stmts: SemStmtIdxRange, f: impl FnOnce(&mut Self)) {
            f(self)
        }

        fn visit_stmt(&mut self, stmt: SemStmtIdx, f: impl FnOnce(&mut Self)) {
            f(self)
        }

        fn visit_stmt_itself(&mut self, stmt: SemStmtIdx) {
            let token_idx_range =
                self.sem_expr_range_region_data[stmt].token_idx_range(self.base.unwrap());
            let text_range = self.ranged_token_sheet.tokens_text_range(token_idx_range);
            self.visits.push(self.text.text_within(text_range));
        }

        fn visit_for_loop_stmt_inner(
            &mut self,
            stmt: SemStmtIdx,
            for_loop_varible_idx: CurrentVariableIdx,
            f: impl Fn(&mut Self),
        ) {
            f(self)
        }

        fn visit_loop(&mut self, stmt: SemStmtIdx, f: impl Fn(&mut Self)) {
            f(self)
        }

        fn visit_branches(&mut self, f: impl Fn(&mut Self)) {
            f(self)
        }

        fn visit_branch(&mut self, f: impl Fn(&mut Self)) {
            f(self)
        }

        fn visit_condition(&mut self, condition: SemCondition, f: impl FnOnce(&mut Self)) {
            f(self)
        }

        fn visit_condition_inner(&mut self, condition: SemCondition) {
            let token_idx_range = condition
                .regional_token_idx_range(
                    self.sem_expr_range_region_data,
                    self.syn_expr_range_region,
                )
                .token_idx_range(self.base.unwrap());
            let text_range = self.ranged_token_sheet.tokens_text_range(token_idx_range);
            self.visits.push(self.text.text_within(text_range));
        }
    }

    impl<'db> SemExprVisitor<'db> {
        fn finish(self) -> Vec<&'db str> {
            self.visits
        }
    }

    DB::ast_rich_test_debug(
        |db, syn_node_region_path: SynNodeRegionPath| match syn_node_region_path.region_path(db) {
            Some(region_path) => {
                let mut visitor = SemExprVisitor::new(region_path, db)?;
                visitor.visit_all(|_| true);
                Some(visitor.finish())
            }
            None => None,
        },
        &AstTestConfig::new(
            "visit_sem_expr",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    )
}
