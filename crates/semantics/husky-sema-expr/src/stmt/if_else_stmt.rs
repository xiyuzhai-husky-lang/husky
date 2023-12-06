use husky_regional_token::{
    ElifRegionalToken, ElseRegionalToken, EolColonRegionalToken, IfRegionalToken,
};

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SemaIfBranch {
    pub if_token: IfRegionalToken,
    pub condition: SemaCondition,
    pub eol_colon_token: EolColonRegionalToken,
    pub stmts: SemaStmtIdxRange,
}

impl SemaIfBranch {
    pub fn eol_colon_token(&self) -> EolColonRegionalToken {
        self.eol_colon_token
    }

    pub fn stmts(&self) -> SemaStmtIdxRange {
        self.stmts
    }

    pub fn if_token(&self) -> IfRegionalToken {
        self.if_token
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
            condition: self.build_sema_condition(*syn_if_branch.condition.as_ref()?),
            eol_colon_token: *syn_if_branch.eol_colon.as_ref()?,
            stmts: self.build_sema_branch(syn_if_branch.stmts, merger),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaElifBranch {
    pub elif_token: ElifRegionalToken,
    pub condition: SemaCondition,
    pub eol_colon_token: EolColonRegionalToken,
    pub stmts: SemaStmtIdxRange,
}

impl SemaElifBranch {
    pub fn eol_colon_token(&self) -> EolColonRegionalToken {
        self.eol_colon_token
    }

    pub fn stmts(&self) -> SemaStmtIdxRange {
        self.stmts
    }

    pub fn elif_regional_token(&self) -> ElifRegionalToken {
        self.elif_token
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
            condition: self.build_sema_condition(*syn_elif_branch.condition.as_ref()?),
            eol_colon_token: *syn_elif_branch.eol_colon.as_ref()?,
            stmts: self.build_sema_branch(syn_elif_branch.stmts, merger),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaElseBranch {
    pub else_regional_token: ElseRegionalToken,
    pub eol_colon_regional_token: EolColonRegionalToken,
    pub stmts: SemaStmtIdxRange,
}

impl SemaElseBranch {
    pub fn stmts(&self) -> SemaStmtIdxRange {
        self.stmts
    }

    pub fn else_regional_token(&self) -> ElseRegionalToken {
        self.else_regional_token
    }

    pub fn eol_colon_regional_token(&self) -> EolColonRegionalToken {
        self.eol_colon_regional_token
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
                if_branch: sema_if_branch,
                elif_branches: sema_elif_branches,
                else_branch: sema_else_branch,
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
            else_regional_token: syn_else_branch.else_token,
            eol_colon_regional_token: *syn_else_branch.eol_colon_token.as_ref()?,
            stmts: self.build_sema_branch(syn_else_branch.stmts, merger),
        })
    }
}
