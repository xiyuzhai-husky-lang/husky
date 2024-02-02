use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclarativeTermMenu1 {
    parent: DeclarativeTermMenu0,
    static_ref_ty: DeclarativeTerm,
    explicit_invariant_ty0_to_trai_ty: CurryDeclarativeTerm,
    explicit_covariant_ty0_to_ty0: CurryDeclarativeTerm,
    explicit_contravariant_ty0_to_ty0: CurryDeclarativeTerm,
    explicit_invariant_ty0_to_ty0: CurryDeclarativeTerm,
    implicit_self_lifetime: SymbolDeclarativeTerm,
    implicit_self_place: SymbolDeclarativeTerm,
}

impl std::ops::Deref for DeclarativeTermMenu1 {
    type Target = DeclarativeTermMenu0;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DeclarativeTermMenu1 {
    pub fn new(db: &::salsa::Db, toolchain: Toolchain, menu0: DeclarativeTermMenu0) -> Self {
        Self {
            static_ref_ty: ApplicationDeclarativeTerm::new(
                db,
                menu0.ref_ty_path(),
                menu0.static_lifetime(),
            )
            .into(),
            explicit_invariant_ty0_to_trai_ty: CurryDeclarativeTerm::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Invariant,
                menu0.ty0().into(),
                menu0.trai_ty().into(),
            ),
            explicit_covariant_ty0_to_ty0: CurryDeclarativeTerm::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Covariant,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_contravariant_ty0_to_ty0: CurryDeclarativeTerm::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Contravariant,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_invariant_ty0_to_ty0: CurryDeclarativeTerm::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Invariant,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            implicit_self_lifetime: SymbolDeclarativeTerm::new(
                db,
                toolchain,
                Ok(menu0.lifetime_ty()),
                DeclarativeTermSymbolIndex::SELF_LIFETIME,
            ),
            implicit_self_place: SymbolDeclarativeTerm::new(
                db,
                toolchain,
                Ok(menu0.place_ty()),
                DeclarativeTermSymbolIndex::SELF_PLACE,
            ),
            parent: menu0,
        }
    }

    pub fn invariant_ty0_to_trai_ty(&self) -> CurryDeclarativeTerm {
        self.explicit_invariant_ty0_to_trai_ty
    }

    pub fn explicit_covariant_ty0_to_ty0(&self) -> CurryDeclarativeTerm {
        self.explicit_covariant_ty0_to_ty0
    }

    pub fn explicit_contravariant_ty0_to_ty0(&self) -> CurryDeclarativeTerm {
        self.explicit_contravariant_ty0_to_ty0
    }

    pub fn ex_inv_ty0_to_ty0(&self) -> CurryDeclarativeTerm {
        self.explicit_invariant_ty0_to_ty0
    }

    pub fn static_ref_ty(&self) -> DeclarativeTerm {
        self.static_ref_ty
    }

    pub fn implicit_self_lifetime(&self) -> SymbolDeclarativeTerm {
        self.implicit_self_lifetime
    }

    pub fn implicit_self_place(&self) -> SymbolDeclarativeTerm {
        self.implicit_self_place
    }
}
