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
    pub elif_token: ElifRegionalToken,
    pub condition: SemaExprResult<SemaExprIdx>,
    pub eol_colon: SemaExprResult<EolRegionalToken>,
    pub stmts: SemaStmtIdxRange,
}

impl SemaElifBranch {
    pub fn condition(&self) -> Result<SemaExprIdx, &SemaExprError> {
        self.condition.as_ref().copied()
    }

    pub fn eol_colon(&self) -> Result<EolRegionalToken, &SemaExprError> {
        self.eol_colon.as_ref().copied()
    }

    pub fn stmts(&self) -> SemaStmtIdxRange {
        self.stmts
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaElseBranch {
    pub else_token: ElseRegionalToken,
    pub eol_colon: SemaExprResult<EolRegionalToken>,
    pub stmts: SemaStmtIdxRange,
}

impl SemaElseBranch {
    pub fn stmts(&self) -> SemaStmtIdxRange {
        self.stmts
    }
}
