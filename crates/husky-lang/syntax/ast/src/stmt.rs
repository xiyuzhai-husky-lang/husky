use word::CustomIdentifier;

use crate::expr::ExprIdx;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Stmt {
    Loop(Loop),
    Branch(BranchStmt),
    Exec {
        expr: ExprIdx,
    },
    Init {
        kind: InitKind,
        varname: CustomIdentifier,
        initial_value: ExprIdx,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Loop {
    For {
        left_bound: ExprIdx,
        right_bound: ExprIdx,
        is_left_shifted: bool,
        is_right_shifted: bool,
        is_incremental: bool,
        fvar_ident: CustomIdentifier,
    },
    ForExt {
        bound: ExprIdx,
        is_shifted: bool,
        is_incremental: bool,
        fvar_ident: CustomIdentifier,
    },
    While {
        condition: ExprIdx,
    },
    DoWhile {
        condition: ExprIdx,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InitKind {
    Let,
    Var,
    Functional,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BranchStmt {
    If { condition: ExprIdx },
    Elif { condition: ExprIdx },
    Else,
}
