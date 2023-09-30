use crate::*;

pub trait TraceDb: salsa::DbWithJar<TraceJar> {}

impl<Db> TraceDb for Db where Db: salsa::DbWithJar<TraceJar> {}

#[salsa::jar(db = TraceDb)]
pub struct TraceJar(
    LazyCallTrace,
    LazyExprTrace,
    LazyStmtTrace,
    EagerCallTrace,
    EagerExprTrace,
    EagerStmtTrace,
    LoopGroupTrace,
);
