use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EagerStmtTracePathData {
    Let { vars: EagerLetStmtVariablesSketch },
    Return,
    If,
    Match,
    ForBetween,
    For,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EagerLetStmtVariablesSketch {
    Single { var_ident: Ident },
    Tuple2 { var_idents: [Ident; 2] },
    Tuple3 { var_idents: [Ident; 3] },
    Other,
}
