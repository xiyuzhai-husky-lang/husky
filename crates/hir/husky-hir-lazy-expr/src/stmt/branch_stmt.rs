use husky_sema_expr::{SemaElifBranch, SemaElseBranch, SemaIfBranch};

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirLazyIfBranch {
    pub condition: HirLazyCondition,
    pub stmts: HirLazyStmtIdxRange,
}

impl ToHirLazy for SemaIfBranch {
    type Output = HirLazyIfBranch;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        HirLazyIfBranch {
            condition: self.condition.to_hir_lazy(builder),
            stmts: self.stmts().to_hir_lazy(builder),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirLazyElifBranch {
    pub condition: HirLazyCondition,
    pub stmts: HirLazyStmtIdxRange,
}

impl ToHirLazy for SemaElifBranch {
    type Output = HirLazyElifBranch;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        HirLazyElifBranch {
            condition: self.condition.to_hir_lazy(builder),
            stmts: self.stmts().to_hir_lazy(builder),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirLazyElseBranch {
    pub stmts: HirLazyStmtIdxRange,
}

impl HirLazyElseBranch {
    pub fn stmts(&self) -> HirLazyStmtIdxRange {
        self.stmts
    }
}

impl ToHirLazy for SemaElseBranch {
    type Output = HirLazyElseBranch;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        HirLazyElseBranch {
            stmts: self.stmts().to_hir_lazy(builder),
        }
    }
}
