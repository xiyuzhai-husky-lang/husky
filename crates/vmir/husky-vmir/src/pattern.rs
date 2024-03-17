pub mod destructive;
pub mod restructive;

use husky_hir_eager_expr::{HirEagerPatternData, HirEagerPatternIdx};

use self::{destructive::VmirDestructivePatternIdx, restructive::VmirRestructivePatternIdx};
use crate::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VmirPatternIdx<LinkageImpl: IsLinkageImpl> {
    /// a restructive version is always needed to tell if pattern is satisfied
    restructive_pattern: VmirRestructivePatternIdx<LinkageImpl>,
    /// only need this for destructive pattern
    destructive_pattern: Option<VmirDestructivePatternIdx<LinkageImpl>>,
}

impl<LinkageImpl: IsLinkageImpl> VmirPatternIdx<LinkageImpl> {
    /// a restructive version is always needed to tell if pattern is satisfied
    pub fn restructive_pattern(&self) -> VmirRestructivePatternIdx<LinkageImpl> {
        self.restructive_pattern
    }

    /// only need this for destructive pattern
    pub fn destructive_pattern(&self) -> Option<VmirDestructivePatternIdx<LinkageImpl>> {
        self.destructive_pattern
    }
}

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for HirEagerPatternIdx {
    type Output = VmirPatternIdx<LinkageImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        let restructive_pattern = builder.build_restructive_pattern(self);
        let destructive_pattern = builder.build_destructive_pattern(self);
        VmirPatternIdx {
            restructive_pattern,
            destructive_pattern,
        }
    }
}
