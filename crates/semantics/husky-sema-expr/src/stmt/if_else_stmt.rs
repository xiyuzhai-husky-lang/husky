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
        Ok(SemaIfBranch {
            if_token: syn_if_branch.if_token,
            condition: self
                .build_sema_expr(*syn_if_branch.condition.as_ref()?, ExpectConditionType),
            eol_colon: *syn_if_branch.eol_colon.as_ref()?,
            stmts: self.build_sema_branch(syn_if_branch.stmts, merger),
        })
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
        syn_elif_branch: &'a SynElifBranch,
        merger: &mut BranchTypeMerger<Expectation>,
    ) -> SynExprResultRef<'a, SemaElifBranch> {
        Ok(SemaElifBranch {
            elif_token: syn_elif_branch.elif_token,
            condition: self
                .build_sema_expr(*syn_elif_branch.condition.as_ref()?, ExpectConditionType),
            eol_colon: *syn_elif_branch.eol_colon.as_ref()?,
            stmts: self.build_sema_branch(syn_elif_branch.stmts, merger),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaElseBranch {
    pub else_token: ElseRegionalToken,
    pub eol_colon: EolRegionalToken,
    pub stmts: SemaStmtIdxRange,
}

impl SemaElseBranch {
    pub fn stmts(&self) -> SemaStmtIdxRange {
        self.stmts
    }
}

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_if_else_stmt(
        &mut self,
        syn_if_branch: &'a SynIfBranch,
        syn_elif_branches: &'a [SynElifBranch],
        syn_else_branch: Option<&'a SynElseBranch>,
        stmt_ty_expectation: impl ExpectFluffyTerm,
    ) -> (
        SemaExprDataResult<SemaStmtData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let mut merger = BranchTypeMerger::new(stmt_ty_expectation);
        let Ok(sema_if_branch) = self.build_sema_if_branch(syn_if_branch, &mut merger) else {
            todo!()
        };
        let Ok(sema_elif_branches) = syn_elif_branches
            .iter()
            .map(|syn_elif_branch| self.build_sema_elif_branch(syn_elif_branch, &mut merger))
            .collect::<SynExprResultRef<Vec<_>>>()
        else {
            todo!()
        };
        let sema_else_branch = if let Some(syn_else_branch) = syn_else_branch {
            let Ok(sema_else_branch) = self.build_sema_else_branch(syn_else_branch, &mut merger)
            else {
                todo!()
            };
            Some(sema_else_branch)
            // merger.visit_branch(self, syn_else_branch);
        } else {
            None
        };
        // exhaustive iff else branch exists;
        (
            Ok(SemaStmtData::IfElse {
                sema_if_branch,
                sema_elif_branches,
                sema_else_branch,
            }),
            merger
                .merge(syn_else_branch.is_some(), self.eth_term_menu())
                .ok_or(DerivedSemaExprTypeError::BranchTypeMerge.into()),
        )
    }

    fn build_sema_else_branch<Expectation: ExpectFluffyTerm>(
        &mut self,
        syn_else_branch: &'a SynElseBranch,
        merger: &mut BranchTypeMerger<Expectation>,
    ) -> SynExprResultRef<'a, SemaElseBranch> {
        Ok(SemaElseBranch {
            else_token: syn_else_branch.else_token,
            eol_colon: *syn_else_branch.eol_colon.as_ref()?,
            stmts: self.build_sema_branch(syn_else_branch.stmts, merger),
        })
    }
}
