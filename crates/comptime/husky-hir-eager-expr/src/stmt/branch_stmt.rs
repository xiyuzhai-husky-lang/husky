use husky_syn_expr::{SynElifBranch, SynElseBranch, SynIfBranch};

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct HirEagerIfBranch {
    pub condition: HirEagerExprIdx,
    pub stmts: HirEagerStmtIdxRange,
}

impl HirEagerIfBranch {
    pub fn condition(&self) -> HirEagerExprIdx {
        self.condition
    }

    pub fn stmts(&self) -> HirEagerStmtIdxRange {
        self.stmts
    }
}

impl ToHirEager for SynIfBranch {
    type Output = HirEagerIfBranch;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerIfBranch {
            condition: self
                .condition
                .as_ref()
                .expect("hir stage no error")
                .to_hir_eager(builder),
            stmts: self
                .stmts()
                .expect("hir stage no error")
                .to_hir_eager(builder),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirEagerElifBranch {
    pub condition: HirEagerExprIdx,
    pub stmts: HirEagerStmtIdxRange,
}

impl HirEagerElifBranch {
    pub fn condition(&self) -> HirEagerExprIdx {
        self.condition
    }

    pub fn stmts(&self) -> HirEagerStmtIdxRange {
        self.stmts
    }
}

impl ToHirEager for SynElifBranch {
    type Output = HirEagerElifBranch;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerElifBranch {
            condition: self
                .condition
                .as_ref()
                .expect("hir stage no error")
                .to_hir_eager(builder),
            stmts: self
                .stmts
                .as_ref()
                .expect("hir stage no error")
                .to_hir_eager(builder),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirEagerElseBranch {
    pub stmts: HirEagerStmtIdxRange,
}

impl HirEagerElseBranch {
    pub fn stmts(&self) -> HirEagerStmtIdxRange {
        self.stmts
    }
}

impl ToHirEager for SynElseBranch {
    type Output = HirEagerElseBranch;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerElseBranch {
            stmts: self
                .stmts()
                .expect("hir stage no error")
                .to_hir_eager(builder),
        }
    }
}
