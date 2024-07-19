use super::*;
use husky_hir_eager_expr::HirEagerCaseBranch;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VmirCaseBranch<LinketImpl: IsLinketImpl> {
    pattern: VmirPattern<LinketImpl>,
    stmts: VmirStmtIdxRange<LinketImpl>,
}

pub type VmirCaseBranches<LinketImpl> = Vec<VmirCaseBranch<LinketImpl>>;

/// # getters
impl<LinketImpl: IsLinketImpl> VmirCaseBranch<LinketImpl> {
    pub fn stmts(&self) -> VmirStmtIdxRange<LinketImpl> {
        self.stmts
    }
}

impl<LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for &HirEagerCaseBranch {
    type Output = VmirCaseBranch<LinketImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinketImpl = LinketImpl>,
    {
        VmirCaseBranch {
            pattern: self.pattern.to_vmir(builder),
            stmts: self.stmts.to_vmir(builder),
        }
    }
}
