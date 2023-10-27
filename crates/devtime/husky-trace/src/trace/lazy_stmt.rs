use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct LazyStmtTracePath {
    #[return_ref]
    pub data: LazyStmtTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyStmtTracePathData {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LazyStmtTraceBiologicalParent {
    LazyCall(LazyCallTrace),
    LazyStmt(LazyStmtTrace),
}

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct LazyStmtTrace {
    #[id]
    pub path: LazyStmtTracePath,
    pub biological_parent: LazyStmtTraceBiologicalParent,
}

impl LazyStmtTrace {
    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        todo!()
    }

    pub fn associated_expr_traces<'a>(
        self,
        db: &'a dyn TraceDb,
    ) -> &'a [(SemaExprIdx, LazyExprTrace)] {
        lazy_stmt_associated_expr_traces(db, self)
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn lazy_stmt_associated_expr_traces(
    db: &dyn TraceDb,
    trace: LazyStmtTrace,
) -> VecPairMap<SemaExprIdx, LazyExprTrace> {
    todo!()
}
