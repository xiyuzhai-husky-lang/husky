use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DecTermMenu1 {
    parent: DecTermMenu0,
    static_ref_ty: DecTerm,
    explicit_invariant_ty0_to_trai_ty: DecCurry,
    explicit_covariant_ty0_to_ty0: DecCurry,
    explicit_contravariant_ty0_to_ty0: DecCurry,
    explicit_invariant_ty0_to_ty0: DecCurry,
    implicit_self_lifetime: DecSymbolicVariable,
    implicit_self_place: DecSymbolicVariable,
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
            static_ref_ty: DecApplication::new(db, menu0.ref_ty_path(), menu0.static_lifetime())
                .into(),
            explicit_invariant_ty0_to_trai_ty: DecCurry::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Invariant,
                menu0.ty0().into(),
                menu0.trai_ty().into(),
            ),
            explicit_covariant_ty0_to_ty0: DecCurry::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Covariant,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_contravariant_ty0_to_ty0: DecCurry::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Contravariant,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_invariant_ty0_to_ty0: DecCurry::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Invariant,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            implicit_self_lifetime: DecSymbolicVariable::new(
                db,
                toolchain,
                Ok(menu0.lifetime_ty()),
                DecSymbolicVariableIndex::SELF_LIFETIME,
            ),
            implicit_self_place: DecSymbolicVariable::new(
                db,
                toolchain,
                Ok(menu0.place_ty()),
                DecSymbolicVariableIndex::SELF_PLACE,
            ),
            parent: menu0,
        }
    }

    pub fn invariant_ty0_to_trai_ty(&self) -> DecCurry {
        self.explicit_invariant_ty0_to_trai_ty
    }

    pub fn explicit_covariant_ty0_to_ty0(&self) -> DecCurry {
        self.explicit_covariant_ty0_to_ty0
    }

    pub fn explicit_contravariant_ty0_to_ty0(&self) -> DecCurry {
        self.explicit_contravariant_ty0_to_ty0
    }

    pub fn ex_inv_ty0_to_ty0(&self) -> DecCurry {
        self.explicit_invariant_ty0_to_ty0
    }

    pub fn static_ref_ty(&self) -> DecTerm {
        self.static_ref_ty
    }

    pub fn auto_self_lifetime(&self) -> DecSymbolicVariable {
        self.implicit_self_lifetime
    }

    pub fn auto_self_place(&self) -> DecSymbolicVariable {
        self.implicit_self_place
    }
}
