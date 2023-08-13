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

impl<'a> HirEagerExprBuilder<'a> {
    pub(super) fn new_if_branch(&mut self, if_branch: &SynIfBranch) -> HirEagerIfBranch {
        HirEagerIfBranch {
            condition: self.new_expr(*if_branch.condition.as_ref().expect("hir stage no error")),
            stmts: self.new_stmts(if_branch.stmts().expect("hir stage no error")),
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

impl<'a> HirEagerExprBuilder<'a> {
    pub(super) fn new_elif_branch(&mut self, elif_branch: &SynElifBranch) -> HirEagerElifBranch {
        HirEagerElifBranch {
            condition: self.new_expr(*elif_branch.condition.as_ref().expect("hir stage no error")),
            stmts: self.new_stmts(*elif_branch.stmts.as_ref().expect("hir stage no error")),
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

impl<'a> HirEagerExprBuilder<'a> {
    pub(super) fn new_else_branch(&mut self, else_branch: &SynElseBranch) -> HirEagerElseBranch {
        HirEagerElseBranch {
            stmts: self.new_stmts(else_branch.stmts().expect("hir stage no error")),
        }
    }
}
