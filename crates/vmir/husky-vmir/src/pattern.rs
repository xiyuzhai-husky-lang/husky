pub mod destructive;
pub mod restructive;

use husky_hir_eager_expr::HirEagerPatternIdx;

use self::{destructive::VmirDestructivePatternIdx, restructive::VmirRestructivePatternIdx};
use crate::*;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VmirPatternIdx<LinkageImpl: IsLinkageImpl> {
    /// a restructive version is needed to tell if pattern is satisfied before destructing
    Destructive(
        VmirRestructivePatternIdx<LinkageImpl>,
        VmirDestructivePatternIdx<LinkageImpl>,
    ),
    Restructive(VmirRestructivePatternIdx<LinkageImpl>),
}

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for HirEagerPatternIdx {
    type Output = VmirPatternIdx<LinkageImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirExprBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        todo!()
    }
}
