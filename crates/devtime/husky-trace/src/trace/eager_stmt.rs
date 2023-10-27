use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct EagerStmtTracePath {
    #[return_ref]
    pub data: EagerStmtTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EagerStmtTracePathData {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerStmtTraceBiologicalParent {
    EagerCall(EagerCallTrace),
    EagerStmt(EagerStmtTrace),
}

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct EagerStmtTrace {
    #[id]
    pub path: EagerStmtTracePath,
    pub biological_parent: EagerStmtTraceBiologicalParent,
}

impl EagerStmtTrace {
    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        todo!()
    }

    pub fn associated_expr_traces<'a>(
        self,
        db: &'a dyn TraceDb,
    ) -> &'a [(SemaExprIdx, EagerExprTrace)] {
        eager_stmt_associated_expr_traces(db, self)
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn eager_stmt_associated_expr_traces(
    db: &dyn TraceDb,
    trace: EagerStmtTrace,
) -> VecPairMap<SemaExprIdx, EagerExprTrace> {
    todo!()
}
