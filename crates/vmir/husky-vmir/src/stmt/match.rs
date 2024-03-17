use super::*;
use husky_hir_eager_expr::HirEagerCaseBranch;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VmirCaseBranch<LinkageImpl: IsLinkageImpl> {
    stmts: VmirStmtIdxRange<LinkageImpl>,
}

pub type VmirCaseBranches<LinkageImpl> = Vec<VmirCaseBranch<LinkageImpl>>;

/// # getters
impl<LinkageImpl: IsLinkageImpl> VmirCaseBranch<LinkageImpl> {
    pub fn stmts(&self) -> ArenaIdxRange<VmirStmtData<LinkageImpl>> {
        self.stmts
    }
}

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for &HirEagerCaseBranch {
    type Output = VmirCaseBranch<LinkageImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        VmirCaseBranch {
            stmts: self.stmts.to_vmir(builder),
        }
    }
}
