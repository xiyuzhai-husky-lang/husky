use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DecTermMenu1 {
    parent: DecTermMenu0,
    static_ref_ty: DecTerm,
    explicit_invariant_ty0_to_trai_ty: CurryDecTerm,
    explicit_covariant_ty0_to_ty0: CurryDecTerm,
    explicit_contravariant_ty0_to_ty0: CurryDecTerm,
    explicit_invariant_ty0_to_ty0: CurryDecTerm,
    implicit_self_lifetime: SymbolDecTerm,
    implicit_self_place: SymbolDecTerm,
}

impl std::ops::Deref for DecTermMenu1 {
    type Target = DecTermMenu0;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DecTermMenu1 {
    pub fn new(db: &::salsa::Db, toolchain: Toolchain, menu0: DecTermMenu0) -> Self {
        Self {
            static_ref_ty: ApplicationDecTerm::new(
                db,
                menu0.ref_ty_path(),
                menu0.static_lifetime(),
            )
            .into(),
            explicit_invariant_ty0_to_trai_ty: CurryDecTerm::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Invariant,
                menu0.ty0().into(),
                menu0.trai_ty().into(),
            ),
            explicit_covariant_ty0_to_ty0: CurryDecTerm::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Covariant,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_contravariant_ty0_to_ty0: CurryDecTerm::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Contravariant,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_invariant_ty0_to_ty0: CurryDecTerm::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Invariant,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            implicit_self_lifetime: SymbolDecTerm::new(
                db,
                toolchain,
                Ok(menu0.lifetime_ty()),
                DecTermSymbolIndex::SELF_LIFETIME,
            ),
            implicit_self_place: SymbolDecTerm::new(
                db,
                toolchain,
                Ok(menu0.place_ty()),
                DecTermSymbolIndex::SELF_PLACE,
            ),
            parent: menu0,
        }
    }

    pub fn invariant_ty0_to_trai_ty(&self) -> CurryDecTerm {
        self.explicit_invariant_ty0_to_trai_ty
    }

    pub fn explicit_covariant_ty0_to_ty0(&self) -> CurryDecTerm {
        self.explicit_covariant_ty0_to_ty0
    }

    pub fn explicit_contravariant_ty0_to_ty0(&self) -> CurryDecTerm {
        self.explicit_contravariant_ty0_to_ty0
    }

    pub fn ex_inv_ty0_to_ty0(&self) -> CurryDecTerm {
        self.explicit_invariant_ty0_to_ty0
    }

    pub fn static_ref_ty(&self) -> DecTerm {
        self.static_ref_ty
    }

    pub fn implicit_self_lifetime(&self) -> SymbolDecTerm {
        self.implicit_self_lifetime
    }

    pub fn implicit_self_place(&self) -> SymbolDecTerm {
        self.implicit_self_place
    }
}
