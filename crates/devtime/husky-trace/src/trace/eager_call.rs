use super::*;

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct EagerCallTrace {
    pub biological_parent: EagerCallTraceBiologicalParent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerCallTraceBiologicalParent {
    EagerExpr(EagerExprTrace),
    LazyExpr(LazyExprTrace),
}
