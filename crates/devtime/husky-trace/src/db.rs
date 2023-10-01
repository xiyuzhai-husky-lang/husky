use crate::*;

pub trait TraceDb: salsa::DbWithJar<TraceJar> {}

impl<Db> TraceDb for Db where Db: salsa::DbWithJar<TraceJar> {}

#[salsa::jar(db = TraceDb)]
pub struct TraceJar(
    ValItemTracePath,
    ValItemTrace,
    LazyCallTracePath,
    LazyCallTrace,
    LazyExprTracePath,
    LazyExprTrace,
    LazyStmtTracePath,
    LazyStmtTrace,
    lazy_stmt_associated_expr_traces,
    EagerCallTracePath,
    EagerCallTrace,
    EagerExprTracePath,
    EagerExprTrace,
    EagerStmtTracePath,
    EagerStmtTrace,
    eager_stmt_associated_expr_traces,
    LoopGroupTracePath,
    LoopGroupTrace,
);
