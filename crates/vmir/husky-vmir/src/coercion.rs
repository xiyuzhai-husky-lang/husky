use crate::*;
use husky_hir_eager_expr::coercion::HirEagerCoercion;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VmirCoercion {
    Trivial,
    Never,
    WrapInSome,
    PlaceToLeash,
    Deref,
}

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for HirEagerCoercion {
    type Output = VmirCoercion;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        match self {
            HirEagerCoercion::Trivial(_) => VmirCoercion::Trivial,
            HirEagerCoercion::Never => VmirCoercion::Never,
            HirEagerCoercion::WrapInSome => VmirCoercion::WrapInSome,
            HirEagerCoercion::PlaceToLeash => VmirCoercion::PlaceToLeash,
            HirEagerCoercion::Deref(_) => VmirCoercion::Deref,
        }
    }
}
