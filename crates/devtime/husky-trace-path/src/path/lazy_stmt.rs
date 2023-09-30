use super::*;

#[salsa::interned(db = TracePathDb, jar = TracePathJar)]
pub struct LazyStmtTracePath {
    pub data: LazyStmtTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LazyStmtTracePathData {
    Let { vars: LazyLetStmtVariablesSketch },
    Return,
    If,
    Match,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LazyLetStmtVariablesSketch {
    Single { var_ident: Ident },
    Tuple2 { var_idents: [Ident; 2] },
    Tuple3 { var_idents: [Ident; 3] },
    Other,
}
