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

impl ToHirLazy for SynIfBranch {
    type Output = HirLazyIfBranch;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        HirLazyIfBranch {
            condition: self
                .condition
                .as_ref()
                .expect("hir stage no error")
                .to_hir_lazy(builder),
            stmts: self
                .stmts()
                .expect("hir stage no error")
                .to_hir_lazy(builder),
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

impl ToHirLazy for SynElifBranch {
    type Output = HirLazyElifBranch;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        HirLazyElifBranch {
            condition: self
                .condition
                .as_ref()
                .expect("hir stage no error")
                .to_hir_lazy(builder),
            stmts: self
                .stmts
                .as_ref()
                .expect("hir stage no error")
                .to_hir_lazy(builder),
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

impl ToHirLazy for SynElseBranch {
    type Output = HirLazyElseBranch;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        HirLazyElseBranch {
            stmts: self
                .stmts()
                .expect("hir stage no error")
                .to_hir_lazy(builder),
        }
    }
}
