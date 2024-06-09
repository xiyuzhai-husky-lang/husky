use super::region::sem_expr_region_from_region_path;
use super::*;
use crate::{SemExprData, SemExprIdx, SemExprRegionData, SemStmtIdx, SemStmtIdxRange};
use husky_entity_path::region::RegionPath;
use husky_entity_tree::{node::ItemSynNodePath, region_path::SynNodeRegionPath};
use husky_text::{HasText, Text};

pub trait VisitSemExpr<'db>: Sized {
    fn db(&self) -> &'db ::salsa::Db;
    fn sem_expr_region_data(&self) -> &'db SemExprRegionData;
    fn visit_expr(&mut self, expr: SemExprIdx, f: impl FnOnce(&mut Self));
    fn visit_expr_inner(&mut self, expr: SemExprIdx);
    fn visit_stmts(&mut self, stmts: SemStmtIdxRange, f: impl FnOnce(&mut Self));
    fn visit_stmt(&mut self, stmt: SemStmtIdx);
    /// final
    fn visit_all(&mut self, f: impl Fn(SynExprRootKind) -> bool) {
        for (expr, root_kind) in self.sem_expr_region_data().sem_expr_roots() {
            if f(root_kind) {
                expr.simulate(self)
            }
        }
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
                | SemExprData::InheritedSynSymbol { .. }
                | SemExprData::CurrentSynSymbol { .. }
                | SemExprData::FrameVarDecl { .. }
                | SemExprData::SelfType(_)
                | SemExprData::SelfValue(_) => (),
                SemExprData::Binary { lopd, ropd, .. } => {
                    lopd.simulate(visitor);
                    ropd.simulate(visitor);
                }
                SemExprData::Be {
                    src,
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
                } => todo!(),
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
                SemExprData::Field {
                    self_argument,
                    ref dispatch,
                    ..
                } => todo!(),
                SemExprData::MethodApplication {
                    self_argument,
                    ref template_arguments,
                    ref items,
                    ..
                } => todo!(),
                SemExprData::MethodFnCall {
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
                SemExprData::MethodGnCall {
                    self_argument,
                    ref method_dynamic_dispatch,
                    ref template_arguments,
                    ref ritchie_parameter_argument_matches,
                    ..
                } => todo!(),
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
                    lbox_regional_token_idx,
                    ref index_sem_list_items,
                    rbox_regional_token_idx,
                    ref index_dynamic_dispatch,
                } => todo!(),
                SemExprData::CompositionWithList {
                    owner, ref items, ..
                } => todo!(),
                SemExprData::NewList { ref items, .. } => todo!(),
                SemExprData::BoxColonList { ref items, .. } => {
                    for item in items {
                        item.sem_expr_idx.simulate(visitor);
                    }
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
                } => todo!(),
                SemExprData::Closure {
                    closure_kind_regional_token_idx,
                    lvert_regional_token_idx,
                    ref parameter_obelisks,
                    rvert_regional_token,
                    return_ty,
                    body,
                } => todo!(),
                SemExprData::Sorry { regional_token_idx } => todo!(),
                SemExprData::Todo { regional_token_idx } => todo!(),
                SemExprData::Unreachable { regional_token_idx } => todo!(),
                SemExprData::NestedBlock { stmts, .. } => todo!(),
            }
            visitor.visit_expr_inner(self)
        });
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
        visitor.visit_stmt(self)
    }
}

#[test]
fn visit_sem_expr_works() {
    struct SemExprVisitor<'db> {
        db: &'db ::salsa::Db,
        text: Text<'db>,
        sem_expr_region_data: &'db SemExprRegionData,
        visits: Vec<String>,
    }

    impl<'db> SemExprVisitor<'db> {
        fn new(region_path: RegionPath, db: &'db ::salsa::Db) -> Option<Self> {
            let module_path = region_path.module_path(db);
            let text = module_path.text(db);
            Some(Self {
                db,
                text,
                sem_expr_region_data: sem_expr_region_from_region_path(region_path, db)?.data(db),
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

        fn visit_expr_inner(&mut self, expr: SemExprIdx) {
            ()
        }

        fn visit_stmts(&mut self, stmts: SemStmtIdxRange, f: impl FnOnce(&mut Self)) {
            f(self)
        }

        fn visit_stmt(&mut self, stmt: SemStmtIdx) {
            ()
        }
    }

    impl<'db> SemExprVisitor<'db> {
        fn finish(self) -> Vec<String> {
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
