use husky_sema_expr::{SemaElifBranch, SemaElseBranch, SemaIfBranch};
use husky_syn_expr::{SynElifBranch, SynElseBranch, SynIfBranch};

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirEagerIfBranch {
    pub condition: HirEagerCondition,
    pub stmts: HirEagerStmtIdxRange,
}

impl HirEagerIfBranch {
    pub fn condition(&self) -> HirEagerCondition {
        self.condition
    }

    pub fn stmts(&self) -> HirEagerStmtIdxRange {
        self.stmts
    }
}

impl ToHirEager for SemaIfBranch {
    type Output = HirEagerIfBranch;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerIfBranch {
            condition: HirEagerCondition(self.condition().to_hir_eager(builder)),
            stmts: self.stmts().to_hir_eager(builder),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirEagerElifBranch {
    pub condition: HirEagerCondition,
    pub stmts: HirEagerStmtIdxRange,
}

impl HirEagerElifBranch {
    pub fn condition(&self) -> HirEagerCondition {
        self.condition
    }

    pub fn stmts(&self) -> HirEagerStmtIdxRange {
        self.stmts
    }
}

impl ToHirEager for SemaElifBranch {
    type Output = HirEagerElifBranch;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerElifBranch {
            condition: HirEagerCondition(self.condition().to_hir_eager(builder)),
            stmts: self.stmts().to_hir_eager(builder),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirEagerElseBranch {
    pub stmts: HirEagerStmtIdxRange,
}

impl HirEagerElseBranch {
    pub fn stmts(&self) -> HirEagerStmtIdxRange {
        self.stmts
    }
}

impl ToHirEager for SemaElseBranch {
    type Output = HirEagerElseBranch;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerElseBranch {
            stmts: self.stmts().to_hir_eager(builder),
        }
    }
}
