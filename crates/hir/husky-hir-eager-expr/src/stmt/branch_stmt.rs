use husky_sema_expr::{SemaCaseBranch, SemaElifBranch, SemaElseBranch, SemaIfBranch};

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerIfBranch {
    pub condition: HirEagerCondition,
    pub stmts: HirEagerStmtIdxRange,
}

impl ToHirEager for SemaIfBranch {
    type Output = HirEagerIfBranch;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerIfBranch {
            condition: self.condition.to_hir_eager(builder),
            stmts: self.stmts().to_hir_eager(builder),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerElifBranch {
    pub condition: HirEagerCondition,
    pub stmts: HirEagerStmtIdxRange,
}

impl ToHirEager for SemaElifBranch {
    type Output = HirEagerElifBranch;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerElifBranch {
            condition: self.condition.to_hir_eager(builder),
            stmts: self.stmts().to_hir_eager(builder),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirEagerElseBranch {
    pub stmts: HirEagerStmtIdxRange,
}

impl ToHirEager for SemaElseBranch {
    type Output = HirEagerElseBranch;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerElseBranch {
            stmts: self.stmts().to_hir_eager(builder),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirEagerCaseBranch {
    pub pattern: HirEagerPatternExprIdx,
    pub stmts: HirEagerStmtIdxRange,
}

impl ToHirEager for SemaCaseBranch {
    type Output = HirEagerCaseBranch;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerCaseBranch {
            pattern: builder.new_pattern_expr(self.case_pattern_sema_obelisk.syn_pattern_root()),
            stmts: self.stmts.to_hir_eager(builder),
        }
    }
}
