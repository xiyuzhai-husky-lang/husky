use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct PreciseTermMenu1 {
    parent: PreciseTermMenu0,
    eval_ref_ty: PreciseTerm,
    static_ref_ty: PreciseTerm,
    explicit_invariant_ty0_to_trai_ty: PreciseTermCurry,
    explicit_covariant_ty0_to_ty0: PreciseTermCurry,
    explicit_contravariant_ty0_to_ty0: PreciseTermCurry,
    explicit_invariant_ty0_to_ty0: PreciseTermCurry,
}

impl std::ops::Deref for PreciseTermMenu1 {
    type Target = PreciseTermMenu0;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl PreciseTermMenu1 {
    pub fn new(db: &dyn PreciseTermDb, _toolchain: Toolchain, menu0: PreciseTermMenu0) -> Self {
        // todo!()
        Self {
            eval_ref_ty: PreciseTermApplication::new(
                db,
                menu0.ref_ty_path().into(),
                menu0.eval_lifetime().into(),
                0,
            )
            .unwrap()
            .into(),
            static_ref_ty: PreciseTermApplication::new(
                db,
                menu0.ref_ty_path(),
                menu0.static_lifetime(),
                0,
            )
            .unwrap()
            .into(),
            explicit_invariant_ty0_to_trai_ty: PreciseTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Invariant,
                None,
                menu0.ty0().into(),
                menu0.trai_ty().into(),
            ),
            explicit_covariant_ty0_to_ty0: PreciseTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_contravariant_ty0_to_ty0: PreciseTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Contravariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_invariant_ty0_to_ty0: PreciseTermCurry::new(
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

    pub fn eval_ref_ty(&self) -> PreciseTerm {
        self.eval_ref_ty
    }

    pub fn static_ref_ty(&self) -> PreciseTerm {
        self.static_ref_ty
    }

    pub fn invariant_ty0_to_trai_ty(&self) -> PreciseTermCurry {
        self.explicit_invariant_ty0_to_trai_ty
    }

    pub fn explicit_covariant_ty0_to_ty0(&self) -> PreciseTermCurry {
        self.explicit_covariant_ty0_to_ty0
    }

    pub fn explicit_contravariant_ty0_to_ty0(&self) -> PreciseTermCurry {
        self.explicit_contravariant_ty0_to_ty0
    }

    pub fn ex_inv_ty0_to_ty0(&self) -> PreciseTermCurry {
        self.explicit_invariant_ty0_to_ty0
    }
}
