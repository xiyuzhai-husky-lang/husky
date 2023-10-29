use super::*;
use husky_hir_lazy_expr::HirLazyExprRegion;
use husky_sema_expr::SemaStmtIdx;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LazyStmtTracePathData {
    Let,
    Return,
    Require,
    Assert,
    Break,
    Eval,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum LazyStmtTraceBiologicalParentPath {
    ValItem(ValItemTracePath),
    LazyCall(LazyCallTracePath),
    LazyStmt(LazyStmtTracePath),
}

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct LazyStmtTracePath {
    pub parent_path: LazyStmtTraceBiologicalParentPath,
    #[return_ref]
    pub path_data: LazyStmtTracePathData,
    pub disambiguator: TracePathDisambiguator<LazyStmtTracePathData>,
}

impl LazyStmtTracePath {
    pub fn new(
        biological_parent_path: impl Into<LazyStmtTraceBiologicalParentPath>,
        path_data: LazyStmtTracePathData,
        registry: &mut TracePathRegistry<LazyStmtTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        LazyStmtTracePath::new_inner(
            db,
            biological_parent_path.into(),
            path_data,
            registry.issue(path_data),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum LazyStmtTraceBiologicalParent {
    ValItem(ValItemTrace),
    LazyCall(LazyCallTrace),
    LazyStmt(LazyStmtTrace),
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct LazyStmtTrace {
    #[id]
    pub path: LazyStmtTracePath,
    pub biological_parent: LazyStmtTraceBiologicalParent,
    pub sema_stmt_idx: SemaStmtIdx,
    #[skip_fmt]
    pub hir_lazy_expr_region: HirLazyExprRegion,
}

impl LazyStmtTrace {
    pub fn new(
        biological_parent: impl Into<LazyStmtTraceBiologicalParent>,
        biological_parent_path: impl Into<LazyStmtTraceBiologicalParentPath>,
        path_data: LazyStmtTracePathData,
        registry: &mut TracePathRegistry<LazyStmtTracePathData>,
        sema_stmt_idx: SemaStmtIdx,
        db: &dyn TraceDb,
    ) -> Self {
        let path = LazyStmtTracePath::new(biological_parent_path, path_data, registry, db);
        LazyStmtTrace::new_inner(db, path, biological_parent.into(), sema_stmt_idx, todo!())
    }

    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        todo!()
    }

    pub fn associated_expr_traces<'a>(self, db: &'a dyn TraceDb) -> &'a [(SemaExprIdx, Trace)] {
        lazy_stmt_associated_expr_traces(db, self)
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn lazy_stmt_associated_expr_traces(
    db: &dyn TraceDb,
    trace: LazyStmtTrace,
) -> VecPairMap<SemaExprIdx, Trace> {
    todo!()
}
