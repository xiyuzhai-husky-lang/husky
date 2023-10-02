mod storage;

pub enum TraceKindProtocol {
    LazyCall,
    LazyExpr,
    LazyStmt,
    EagerCall,
    EagerExpr,
    EagerStmt,
}

pub struct TraceViewData {
    tokens_data: Vec<TraceViewTokenData>,
}

pub struct TraceViewTokenData {}
