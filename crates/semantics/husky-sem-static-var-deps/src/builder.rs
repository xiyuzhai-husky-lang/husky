use crate::{region::SemStaticVarDepsRegion, static_var_deps::SemStaticVarDeps};
use husky_entity_path::{path::ItemPath, region::RegionPath};
use husky_sem_expr::{
    helpers::{region::sem_expr_region_from_region_path, visitor::VisitSemExpr},
    stmt::condition::SemCondition,
    SemExprIdx, SemExprMap, SemExprRegionData, SemStmtIdx, SemStmtIdxRange,
};
use husky_syn_expr::context::SynExprRootKind;
use vec_like::OrderedSmallVecSet;

pub(crate) struct SemStaticVarDepsBuilder<'db, 'a, F>
where
    F: Fn(ItemPath) -> &'a SemStaticVarDeps,
{
    db: &'db ::salsa::Db,
    sem_expr_region_data: &'db SemExprRegionData,
    static_mut_deps_table: SemExprMap<OrderedSmallVecSet<(), 4>>,
    f: F,
}

/// # constructor
impl<'db, 'a, F> SemStaticVarDepsBuilder<'db, 'a, F>
where
    F: Fn(ItemPath) -> &'a SemStaticVarDeps,
{
    pub(crate) fn new(db: &'db ::salsa::Db, region_path: RegionPath, f: F) -> Option<Self> {
        let sem_expr_region_data = sem_expr_region_from_region_path(region_path, db)?.data(db);
        Some(Self {
            db,
            sem_expr_region_data,
            static_mut_deps_table: SemExprMap::new(sem_expr_region_data.sem_expr_arena()),
            f,
        })
    }
}

/// # getters
impl<'db, 'a, F> SemStaticVarDepsBuilder<'db, 'a, F> where F: Fn(ItemPath) -> &'a SemStaticVarDeps {}

/// # actions
impl<'db, 'a, F> SemStaticVarDepsBuilder<'db, 'a, F>
where
    F: Fn(ItemPath) -> &'a SemStaticVarDeps,
{
    pub(crate) fn build_root(&mut self, root_kind: SynExprRootKind) -> SemStaticVarDeps {
        self.sem_expr_region_data.sem_expr_roots();
        todo!()
    }

    fn finish(self) -> SemStaticVarDepsRegion {
        todo!()
    }
}

impl<'db, 'a, F> VisitSemExpr<'db> for SemStaticVarDepsBuilder<'db, 'a, F>
where
    F: Fn(ItemPath) -> &'a SemStaticVarDeps,
{
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
        todo!()
    }

    fn visit_stmts(&mut self, stmts: SemStmtIdxRange, f: impl FnOnce(&mut Self)) {
        f(self)
    }

    fn visit_stmt(&mut self, stmt: SemStmtIdx, f: impl FnOnce(&mut Self)) {
        f(self)
    }

    fn visit_stmt_inner(&mut self, stmt: SemStmtIdx) {
        todo!()
    }

    fn visit_loop(&mut self, stmt: SemStmtIdx, f: impl Fn(&mut Self)) {
        todo!()
    }

    fn visit_branches(&mut self, f: impl Fn(&mut Self)) {
        f(self)
    }

    fn visit_branch(&mut self, f: impl Fn(&mut Self)) {
        f(self)
    }

    fn visit_condition(&mut self, condition: SemCondition, f: impl FnOnce(&mut Self)) {
        ()
    }

    fn visit_condition_inner(&mut self, condition: SemCondition) {
        ()
    }
}
