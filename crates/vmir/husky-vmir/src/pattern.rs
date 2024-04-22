pub mod destructive;
pub mod restructive;

use self::{destructive::VmirDestructivePattern, restructive::VmirRestructivePattern};
use crate::*;
use husky_hir_eager_expr::{HirEagerPatternData, HirEagerPatternIdx};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VmirPattern<LinkageImpl: IsLinkageImpl> {
    /// a restructive version is always needed to tell if pattern is satisfied
    restructive_pattern: VmirRestructivePattern<LinkageImpl>,
    /// only need this for destructive pattern
    destructive_pattern: Option<VmirDestructivePattern<LinkageImpl>>,
}

impl<LinkageImpl: IsLinkageImpl> VmirPattern<LinkageImpl> {
    /// a restructive version is always needed to tell if pattern is satisfied
    pub fn restructive_pattern(self) -> VmirRestructivePattern<LinkageImpl> {
        self.restructive_pattern
    }

    /// only need this for destructive pattern
    pub fn destructive_pattern(self) -> Option<VmirDestructivePattern<LinkageImpl>> {
        self.destructive_pattern
    }
}

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for HirEagerPatternIdx {
    type Output = VmirPattern<LinkageImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        let restructive_pattern = builder.build_restructive_pattern(self);
        let destructive_pattern = builder.build_destructive_pattern(self);
        VmirPattern {
            restructive_pattern,
            destructive_pattern,
        }
    }
}
