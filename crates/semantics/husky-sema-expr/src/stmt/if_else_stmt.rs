use husky_regional_token::{ElifRegionalToken, ElseRegionalToken, IfRegionalToken};

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SemaIfBranch {
    if_token: IfRegionalToken,
    condition: SemaExprIdx,
    eol_colon: EolRegionalToken,
    stmts: SemaStmtIdxRange,
}

impl SemaIfBranch {
    pub fn condition(&self) -> SemaExprIdx {
        self.condition
    }

    pub fn eol_colon_token(&self) -> EolRegionalToken {
        self.eol_colon
    }

    pub fn stmts(&self) -> SemaStmtIdxRange {
        self.stmts
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaElifBranch {
    elif_token: ElifRegionalToken,
    condition: SemaExprIdx,
    eol_colon: EolRegionalToken,
    stmts: SemaStmtIdxRange,
}

impl SemaElifBranch {
    pub fn condition(&self) -> SemaExprIdx {
        self.condition
    }

    pub fn eol_colon(&self) -> EolRegionalToken {
        self.eol_colon
    }

    pub fn stmts(&self) -> SemaStmtIdxRange {
        self.stmts
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaElseBranch {
    pub else_token: ElseRegionalToken,
    pub eol_colon: SemaExprTypeResult<EolRegionalToken>,
    pub stmts: SemaStmtIdxRange,
}

impl SemaElseBranch {
    pub fn stmts(&self) -> SemaStmtIdxRange {
        self.stmts
    }
}
