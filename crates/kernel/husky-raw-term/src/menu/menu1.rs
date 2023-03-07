use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct RawTermMenu1 {
    parent: RawTermMenu0,
    eval_ref_ty: RawTerm,
    static_ref_ty: RawTerm,
    explicit_invariant_ty0_to_trai_ty: RawTermCurry,
    explicit_covariant_ty0_to_ty0: RawTermCurry,
    explicit_contravariant_ty0_to_ty0: RawTermCurry,
    explicit_invariant_ty0_to_ty0: RawTermCurry,
}

impl std::ops::Deref for RawTermMenu1 {
    type Target = RawTermMenu0;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl RawTermMenu1 {
    pub fn new(db: &dyn RawTermDb, _toolchain: Toolchain, menu0: RawTermMenu0) -> Self {
        // todo!()
        Self {
            eval_ref_ty: RawTermApplication::new(db, menu0.ref_ty_path(), menu0.eval_lifetime())
                .into(),
            static_ref_ty: RawTermApplication::new(
                db,
                menu0.ref_ty_path(),
                menu0.static_lifetime(),
            )
            .into(),
            explicit_invariant_ty0_to_trai_ty: RawTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Invariant,
                None,
                menu0.ty0().into(),
                menu0.trai_ty().into(),
            ),
            explicit_covariant_ty0_to_ty0: RawTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_contravariant_ty0_to_ty0: RawTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Contravariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_invariant_ty0_to_ty0: RawTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Invariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            parent: menu0,
        }
    }

    pub fn eval_ref_ty(&self) -> RawTerm {
        self.eval_ref_ty
    }

    pub fn static_ref_ty(&self) -> RawTerm {
        self.static_ref_ty
    }

    pub fn invariant_ty0_to_trai_ty(&self) -> RawTermCurry {
        self.explicit_invariant_ty0_to_trai_ty
    }

    pub fn explicit_covariant_ty0_to_ty0(&self) -> RawTermCurry {
        self.explicit_covariant_ty0_to_ty0
    }

    pub fn explicit_contravariant_ty0_to_ty0(&self) -> RawTermCurry {
        self.explicit_contravariant_ty0_to_ty0
    }

    pub fn ex_inv_ty0_to_ty0(&self) -> RawTermCurry {
        self.explicit_invariant_ty0_to_ty0
    }
}
