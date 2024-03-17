use super::*;
use husky_hir_eager_expr::{HirEagerElifBranch, HirEagerElseBranch, HirEagerIfBranch};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VmirIfBranch<LinkageImpl: IsLinkageImpl> {
    condition: VmirCondition<LinkageImpl>,
    stmts: VmirStmtIdxRange<LinkageImpl>,
}

/// # getters
impl<LinkageImpl: IsLinkageImpl> VmirIfBranch<LinkageImpl> {
    pub fn stmts(&self) -> VmirStmtIdxRange<LinkageImpl> {
        self.stmts
    }
}

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for &HirEagerIfBranch {
    type Output = VmirIfBranch<LinkageImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        VmirIfBranch {
            condition: self.condition.to_vmir(builder),
            stmts: self.stmts.to_vmir(builder),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VmirElifBranch<LinkageImpl: IsLinkageImpl> {
    condition: VmirCondition<LinkageImpl>,
    stmts: VmirStmtIdxRange<LinkageImpl>,
}

pub type VmirElifBranchs<LinkageImpl> = Vec<VmirElifBranch<LinkageImpl>>;

/// # getters
impl<LinkageImpl: IsLinkageImpl> VmirElifBranch<LinkageImpl> {
    pub fn stmts(&self) -> VmirStmtIdxRange<LinkageImpl> {
        self.stmts
    }
}

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for &HirEagerElifBranch {
    type Output = VmirElifBranch<LinkageImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        VmirElifBranch {
            condition: self.condition.to_vmir(builder),
            stmts: self.stmts.to_vmir(builder),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VmirElseBranch<LinkageImpl: IsLinkageImpl> {
    stmts: VmirStmtIdxRange<LinkageImpl>,
}

/// # getters
impl<LinkageImpl: IsLinkageImpl> VmirElseBranch<LinkageImpl> {
    pub fn stmts(&self) -> VmirStmtIdxRange<LinkageImpl> {
        self.stmts
    }
}

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for HirEagerElseBranch {
    type Output = VmirElseBranch<LinkageImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        VmirElseBranch {
            stmts: self.stmts.to_vmir(builder),
        }
    }
}
