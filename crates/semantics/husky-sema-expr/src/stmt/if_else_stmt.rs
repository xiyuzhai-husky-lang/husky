use husky_regional_token::{ElifRegionalToken, ElseRegionalToken, IfRegionalToken};

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SemaIfBranch {
    if_token: IfRegionalToken,
    condition: SemaExprIdx,
    eol_colon: EolRegionalToken,
    stmts: SemaStmtIdxRange,
}

impl SemaIfBranch {
    pub fn condition(&self) -> SemaExprIdx {
        self.condition
    }

    pub fn eol_colon_token(&self) -> EolRegionalToken {
        self.eol_colon
    }

    pub fn stmts(&self) -> SemaStmtIdxRange {
        self.stmts
    }
}

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_sema_if_branch<Expectation: ExpectFluffyTerm>(
        &mut self,
        syn_if_branch: &'a SynIfBranch,
        merger: &mut BranchTypeMerger<Expectation>,
    ) -> SynExprResultRef<'a, SemaIfBranch> {
        self.build_sema_expr_with_its_ty_returned(
            *syn_if_branch.condition.as_ref()?,
            ExpectConditionType,
        );
        // merger.visit_branch(self, if_branch.stmts);
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaElifBranch {
    elif_token: ElifRegionalToken,
    condition: SemaExprIdx,
    eol_colon: EolRegionalToken,
    stmts: SemaStmtIdxRange,
}

impl SemaElifBranch {
    pub fn condition(&self) -> SemaExprIdx {
        self.condition
    }

    pub fn eol_colon(&self) -> EolRegionalToken {
        self.eol_colon
    }

    pub fn stmts(&self) -> SemaStmtIdxRange {
        self.stmts
    }
}

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_sema_elif_branch<Expectation: ExpectFluffyTerm>(
        &mut self,
        syn_elif_branch: &SynElifBranch,
        merger: &mut BranchTypeMerger<Expectation>,
    ) -> SemaElifBranch {
        syn_elif_branch
            .condition
            .as_ref()
            .copied()
            .map(|condition| {
                self.build_sema_expr_with_its_ty_returned(condition, ExpectConditionType)
            });
        // merger.visit_branch(self, if_branch.stmts);
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaElseBranch {
    pub else_token: ElseRegionalToken,
    pub eol_colon: SemaExprTypeResult<EolRegionalToken>,
    pub stmts: SemaStmtIdxRange,
}

impl SemaElseBranch {
    pub fn stmts(&self) -> SemaStmtIdxRange {
        self.stmts
    }
}

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_sema_else_branch<Expectation: ExpectFluffyTerm>(
        &mut self,
        syn_else_branch: &SynElseBranch,
        merger: &mut BranchTypeMerger<Expectation>,
    ) -> SemaElseBranch {
        // merger.visit_branch(self, if_branch.stmts);
        todo!()
    }
}
