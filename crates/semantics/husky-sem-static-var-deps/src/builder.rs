use husky_sem_expr::{
    helpers::visitor::VisitSemExpr, stmt::condition::SemCondition, SemExprIdx, SemExprMap,
    SemExprRegionData, SemStmtIdx, SemStmtIdxRange,
};
use vec_like::OrderedSmallVecSet;

pub(crate) struct SemStaticVarDepsBuilder<'db> {
    db: &'db ::salsa::Db,
    sem_expr_region_data: &'db SemExprRegionData,
    static_mut_deps_table: SemExprMap<OrderedSmallVecSet<(), 4>>,
}

/// # constructor
impl<'db> SemStaticVarDepsBuilder<'db> {
    fn new(db: &'db ::salsa::Db) -> Self {
        Self {
            db: todo!(),
            sem_expr_region_data: todo!(),
            static_mut_deps_table: todo!(),
        }
    }
}

/// # getters
impl<'db> SemStaticVarDepsBuilder<'db> {}

/// # actions
impl<'db> SemStaticVarDepsBuilder<'db> {
    fn finish(self) -> () {
        todo!()
    }
}

impl<'db> VisitSemExpr<'db> for SemStaticVarDepsBuilder<'db> {
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

    fn visit_condition(&mut self, condition: SemCondition, f: impl FnOnce(&mut Self)) {
        ()
    }

    fn visit_condition_inner(&mut self, condition: SemCondition) {
        ()
    }
}
