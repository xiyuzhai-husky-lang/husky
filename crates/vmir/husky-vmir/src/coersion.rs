use crate::*;
use husky_hir_eager_expr::coersion::HirEagerCoersion;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VmirCoersion {
    Trivial,
    Never,
    WrapInSome,
    PlaceToLeash,
    Deref,
}

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for HirEagerCoersion {
    type Output = VmirCoersion;

    fn to_vmir(self, builder: &mut VmirExprBuilder<LinkageImpl>) -> Self::Output {
        match self {
            HirEagerCoersion::Trivial(_) => VmirCoersion::Trivial,
            HirEagerCoersion::Never => VmirCoersion::Never,
            HirEagerCoersion::WrapInSome => VmirCoersion::WrapInSome,
            HirEagerCoersion::PlaceToLeash => VmirCoersion::PlaceToLeash,
            HirEagerCoersion::Deref(_) => VmirCoersion::Deref,
        }
    }
}
