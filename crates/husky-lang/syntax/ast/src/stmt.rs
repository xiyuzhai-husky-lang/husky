use word::CustomIdentifier;

use crate::{expr::RawExprIdx, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawStmt {
    Loop(LoopRawStmt),
    Branch(BranchRawStmt),
    Exec(RawExprIdx),
    Init {
        kind: InitKind,
        varname: CustomIdentifier,
        initial_value: RawExprIdx,
    },
    Return(RawExprIdx),
    Assert(RawExprIdx),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LoopRawStmt {
    For {
        left_bound: RawExprIdx,
        right_bound: RawExprIdx,
        is_left_shifted: bool,
        is_right_shifted: bool,
        is_incremental: bool,
        fvar_ident: CustomIdentifier,
    },
    ForExt {
        bound: RawExprIdx,
        is_shifted: bool,
        is_incremental: bool,
        fvar_ident: CustomIdentifier,
    },
    While {
        condition: RawExprIdx,
    },
    DoWhile {
        condition: RawExprIdx,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BranchRawStmt {
    If { condition: RawExprIdx },
    Elif { condition: RawExprIdx },
    Else,
}
