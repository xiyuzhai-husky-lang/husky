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
