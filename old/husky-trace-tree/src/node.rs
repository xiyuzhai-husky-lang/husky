pub struct TraceNode {
    data: TraceNodeData,
}

pub enum TraceNodeData {
    Expr,
    Stmt,
}
