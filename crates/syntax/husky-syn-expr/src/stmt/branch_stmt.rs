use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct IfBranch {
    pub if_token: IfToken,
    pub condition: SynExprResult<SynExprIdx>,
    pub eol_colon: SynExprResult<EolToken>,
    pub block: SynExprResult<SynStmtIdxRange>,
}

impl IfBranch {
    pub fn condition(&self) -> Result<&SynExprIdx, &ExprError> {
        self.condition.as_ref()
    }

    pub fn eol_colon_token(&self) -> Result<&EolToken, &ExprError> {
        self.eol_colon.as_ref()
    }

    pub fn block(&self) -> Result<SynStmtIdxRange, &ExprError> {
        self.block.as_ref().copied()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ElifBranch {
    pub elif_token: ElifToken,
    pub condition: SynExprResult<SynExprIdx>,
    pub eol_colon: SynExprResult<EolToken>,
    pub block: SynExprResult<SynStmtIdxRange>,
}

impl ElifBranch {
    pub fn condition(&self) -> Result<&SynExprIdx, &ExprError> {
        self.condition.as_ref()
    }

    pub fn eol_colon(&self) -> Result<&EolToken, &ExprError> {
        self.eol_colon.as_ref()
    }

    pub fn block(&self) -> Result<SynStmtIdxRange, &ExprError> {
        self.block.as_ref().copied()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ElseBranch {
    pub else_token: ElseToken,
    pub eol_colon: SynExprResult<EolToken>,
    pub block: SynExprResult<SynStmtIdxRange>,
}

impl ElseBranch {
    pub fn block(&self) -> Result<SynStmtIdxRange, &ExprError> {
        self.block.as_ref().copied()
    }
}
