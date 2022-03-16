#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Func {
    stmts: Vec<FuncStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncStmt {
    kind: FuncStmtKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncStmtKind {
    Init,
}
