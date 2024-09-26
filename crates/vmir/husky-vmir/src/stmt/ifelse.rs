use super::*;
use husky_hir_eager_expr::{HirEagerElifBranch, HirEagerElseBranch, HirEagerIfBranch};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VmirIfBranch<LinketImpl: IsLinketImpl> {
    condition: VmirCondition<LinketImpl>,
    stmts: VmirStmtIdxRange<LinketImpl>,
}

/// # getters
impl<LinketImpl: IsLinketImpl> VmirIfBranch<LinketImpl> {
    pub fn stmts(&self) -> VmirStmtIdxRange<LinketImpl> {
        self.stmts
    }
}

impl<LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for &HirEagerIfBranch {
    type Output = VmirIfBranch<LinketImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinketImpl = LinketImpl>,
    {
        VmirIfBranch {
            condition: self.condition.to_vmir(builder),
            stmts: self.stmts.to_vmir(builder),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VmirElifBranch<LinketImpl: IsLinketImpl> {
    condition: VmirCondition<LinketImpl>,
    stmts: VmirStmtIdxRange<LinketImpl>,
}

pub type VmirElifBranchs<LinketImpl> = Vec<VmirElifBranch<LinketImpl>>;

/// # getters
impl<LinketImpl: IsLinketImpl> VmirElifBranch<LinketImpl> {
    pub fn stmts(&self) -> VmirStmtIdxRange<LinketImpl> {
        self.stmts
    }
}

impl<LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for &HirEagerElifBranch {
    type Output = VmirElifBranch<LinketImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinketImpl = LinketImpl>,
    {
        VmirElifBranch {
            condition: self.condition.to_vmir(builder),
            stmts: self.stmts.to_vmir(builder),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VmirElseBranch<LinketImpl: IsLinketImpl> {
    stmts: VmirStmtIdxRange<LinketImpl>,
}

/// # getters
impl<LinketImpl: IsLinketImpl> VmirElseBranch<LinketImpl> {
    pub fn stmts(&self) -> VmirStmtIdxRange<LinketImpl> {
        self.stmts
    }
}

impl<LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for HirEagerElseBranch {
    type Output = VmirElseBranch<LinketImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinketImpl = LinketImpl>,
    {
        VmirElseBranch {
            stmts: self.stmts.to_vmir(builder),
        }
    }
}

impl<LinketImpl: IsLinketImpl> VmirStmtIdx<LinketImpl> {
    pub(super) fn eval_if_else<'comptime>(
        self,
        if_branch: &VmirIfBranch<LinketImpl>,
        elif_branches: &VmirElifBranchs<LinketImpl>,
        else_branch: Option<&VmirElseBranch<LinketImpl>>,
        ctx: &mut impl EvalVmir<'comptime, LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<LinketImpl> {
        if if_branch.condition.eval(ctx)? {
            return if_branch.stmts.eval(ctx);
        }
        for elif_branch in elif_branches {
            if elif_branch.condition.eval(ctx)? {
                return elif_branch.stmts.eval(ctx);
            }
        }
        if let Some(else_branch) = else_branch {
            else_branch.stmts.eval(ctx)
        } else {
            VmControlFlow::Continue(().into())
        }
    }
}
