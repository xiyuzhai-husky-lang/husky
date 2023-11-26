use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclarativeTermMenu1 {
    parent: DeclarativeTermMenu0,
    static_ref_ty: DeclarativeTerm,
    explicit_invariant_ty0_to_trai_ty: DeclarativeTermCurry,
    explicit_covariant_ty0_to_ty0: DeclarativeTermCurry,
    explicit_contravariant_ty0_to_ty0: DeclarativeTermCurry,
    explicit_invariant_ty0_to_ty0: DeclarativeTermCurry,
    implicit_self_lifetime: DeclarativeTermSymbol,
    implicit_self_place: DeclarativeTermSymbol,
}

impl std::ops::Deref for DeclarativeTermMenu1 {
    type Target = DeclarativeTermMenu0;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DeclarativeTermMenu1 {
    pub fn new(db: &::salsa::Db, _toolchain: Toolchain, menu0: DeclarativeTermMenu0) -> Self {
        Self {
            static_ref_ty: DeclarativeTermExplicitApplication::new(
                db,
                menu0.ref_ty_path(),
                menu0.static_lifetime(),
            )
            .into(),
            explicit_invariant_ty0_to_trai_ty: DeclarativeTermCurry::new_nondependent(
                db,
                CurryKind::Explicit,
                Variance::Invariant,
                menu0.ty0().into(),
                menu0.trai_ty().into(),
            ),
            explicit_covariant_ty0_to_ty0: DeclarativeTermCurry::new_nondependent(
                db,
                CurryKind::Explicit,
                Variance::Covariant,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_contravariant_ty0_to_ty0: DeclarativeTermCurry::new_nondependent(
                db,
                CurryKind::Explicit,
                Variance::Contravariant,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_invariant_ty0_to_ty0: DeclarativeTermCurry::new_nondependent(
                db,
                CurryKind::Explicit,
                Variance::Invariant,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            implicit_self_lifetime: DeclarativeTermSymbol::new(
                db,
                Ok(menu0.lifetime_ty()),
                DeclarativeTermSymbolIndex::SELF_LIFETIME,
            ),
            implicit_self_place: DeclarativeTermSymbol::new(
                db,
                Ok(menu0.place_ty()),
                DeclarativeTermSymbolIndex::SELF_PLACE,
            ),
            parent: menu0,
        }
    }

    pub fn invariant_ty0_to_trai_ty(&self) -> DeclarativeTermCurry {
        self.explicit_invariant_ty0_to_trai_ty
    }

    pub fn explicit_covariant_ty0_to_ty0(&self) -> DeclarativeTermCurry {
        self.explicit_covariant_ty0_to_ty0
    }

    pub fn explicit_contravariant_ty0_to_ty0(&self) -> DeclarativeTermCurry {
        self.explicit_contravariant_ty0_to_ty0
    }

    pub fn ex_inv_ty0_to_ty0(&self) -> DeclarativeTermCurry {
        self.explicit_invariant_ty0_to_ty0
    }

    pub fn static_ref_ty(&self) -> DeclarativeTerm {
        self.static_ref_ty
    }

    pub fn implicit_self_lifetime(&self) -> DeclarativeTermSymbol {
        self.implicit_self_lifetime
    }

    pub fn implicit_self_place(&self) -> DeclarativeTermSymbol {
        self.implicit_self_place
    }
}
