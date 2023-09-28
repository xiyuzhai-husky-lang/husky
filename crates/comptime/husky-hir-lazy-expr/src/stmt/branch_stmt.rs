use husky_sema_expr::{SemaElifBranch, SemaElseBranch, SemaIfBranch};
use husky_syn_expr::{SynElifBranch, SynElseBranch, SynIfBranch};

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct HirLazyIfBranch {
    pub condition: HirLazyExprIdx,
    pub stmts: HirLazyStmtIdxRange,
}

impl HirLazyIfBranch {
    pub fn condition(&self) -> HirLazyExprIdx {
        self.condition
    }

    pub fn stmts(&self) -> HirLazyStmtIdxRange {
        self.stmts
    }
}

impl ToHirLazy for SemaIfBranch {
    type Output = HirLazyIfBranch;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        HirLazyIfBranch {
            condition: self.condition().to_hir_lazy(builder),
            stmts: self.stmts().to_hir_lazy(builder),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirLazyElifBranch {
    pub condition: HirLazyExprIdx,
    pub stmts: HirLazyStmtIdxRange,
}

impl HirLazyElifBranch {
    pub fn condition(&self) -> HirLazyExprIdx {
        self.condition
    }

    pub fn stmts(&self) -> HirLazyStmtIdxRange {
        self.stmts
    }
}

impl ToHirLazy for SemaElifBranch {
    type Output = HirLazyElifBranch;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        HirLazyElifBranch {
            condition: self.condition().to_hir_lazy(builder),
            stmts: self.stmts().to_hir_lazy(builder),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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
