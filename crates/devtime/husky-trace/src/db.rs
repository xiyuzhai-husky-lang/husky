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
    EagerCallTracePath,
    EagerCallTrace,
    EagerExprTracePath,
    EagerExprTrace,
    EagerStmtTracePath,
    EagerStmtTrace,
    LoopGroupTracePath,
    LoopGroupTrace,
);
