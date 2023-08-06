use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct HirEagerIfBranch {
    pub condition: HirEagerExprIdx,
    pub block: HirEagerStmtIdxRange,
}

impl HirEagerIfBranch {
    pub fn condition(&self) -> HirEagerExprIdx {
        self.condition
    }

    pub fn block(&self) -> HirEagerStmtIdxRange {
        self.block
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirEagerElifBranch {
    pub condition: HirEagerExprIdx,
    pub block: HirEagerStmtIdxRange,
}

impl HirEagerElifBranch {
    pub fn condition(&self) -> HirEagerExprIdx {
        self.condition
    }

    pub fn block(&self) -> HirEagerStmtIdxRange {
        self.block
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirEagerElseBranch {
    pub block: HirEagerStmtIdxRange,
}

impl HirEagerElseBranch {
    pub fn block(&self) -> HirEagerStmtIdxRange {
        self.block
    }
}
