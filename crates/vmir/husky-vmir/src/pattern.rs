pub mod destructive;
pub mod restructive;

use self::{
    destructive::VmirDestructivePattern, eval::EvalVmir, restructive::VmirRestructivePattern,
};
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

impl<'comptime, LinkageImpl: IsLinkageImpl> VmirPattern<LinkageImpl> {
    pub(crate) fn take_value(
        self,
        value: LinkageImpl::Value,
        ctx: &mut impl EvalVmir<'comptime, LinkageImpl>,
    ) {
        match self.destructive_pattern {
            Some(destructive_pattern) => match destructive_pattern {
                VmirDestructivePattern::Default(place) => match place {
                    Some(place) => ctx.init_place(place, value),
                    None => (),
                },
                VmirDestructivePattern::Or(_) => todo!(),
                VmirDestructivePattern::Other(_) => todo!(),
            },
            None => todo!(),
        }
    }
}
