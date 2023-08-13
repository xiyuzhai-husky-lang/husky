use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SynIfBranch {
    pub if_token: IfToken,
    pub condition: SynExprResult<SynExprIdx>,
    pub eol_colon: SynExprResult<EolToken>,
    pub stmts: SynExprResult<SynStmtIdxRange>,
}

impl SynIfBranch {
    pub fn condition(&self) -> Result<SynExprIdx, &ExprError> {
        self.condition.as_ref().copied()
    }

    pub fn eol_colon_token(&self) -> Result<EolToken, &ExprError> {
        self.eol_colon.as_ref().copied()
    }

    pub fn stmts(&self) -> Result<SynStmtIdxRange, &ExprError> {
        self.stmts.as_ref().copied()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SynElifBranch {
    pub elif_token: ElifToken,
    pub condition: SynExprResult<SynExprIdx>,
    pub eol_colon: SynExprResult<EolToken>,
    pub stmts: SynExprResult<SynStmtIdxRange>,
}

impl SynElifBranch {
    pub fn condition(&self) -> Result<SynExprIdx, &ExprError> {
        self.condition.as_ref().copied()
    }

    pub fn eol_colon(&self) -> Result<EolToken, &ExprError> {
        self.eol_colon.as_ref().copied()
    }

    pub fn stmts(&self) -> Result<SynStmtIdxRange, &ExprError> {
        self.stmts.as_ref().copied()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SynElseBranch {
    pub else_token: ElseToken,
    pub eol_colon: SynExprResult<EolToken>,
    pub stmts: SynExprResult<SynStmtIdxRange>,
}

impl SynElseBranch {
    pub fn stmts(&self) -> Result<SynStmtIdxRange, &ExprError> {
        self.stmts.as_ref().copied()
    }
}
