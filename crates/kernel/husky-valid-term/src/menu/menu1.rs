use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ValidTermMenu1 {
    parent: ValidTermMenu0,
    eval_ref_ty: ValidTerm,
    static_ref_ty: ValidTerm,
    explicit_invariant_ty0_to_trai_ty: ValidTermCurry,
    explicit_covariant_ty0_to_ty0: ValidTermCurry,
    explicit_contravariant_ty0_to_ty0: ValidTermCurry,
    explicit_invariant_ty0_to_ty0: ValidTermCurry,
}

impl std::ops::Deref for ValidTermMenu1 {
    type Target = ValidTermMenu0;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl ValidTermMenu1 {
    pub fn new(db: &dyn ValidTermDb, _toolchain: Toolchain, menu0: ValidTermMenu0) -> Self {
        // todo!()
        Self {
            eval_ref_ty: ValidTermApplication::new(
                db,
                menu0.ref_ty_path().into(),
                menu0.eval_lifetime().into(),
                0,
            )
            .unwrap()
            .into(),
            static_ref_ty: ValidTermApplication::new(
                db,
                menu0.ref_ty_path(),
                menu0.static_lifetime(),
                0,
            )
            .unwrap()
            .into(),
            explicit_invariant_ty0_to_trai_ty: ValidTermCurry::new(
                db,
                ValidTermCurryKind::Explicit,
                Variance::Invariant,
                None,
                menu0.ty0().into(),
                menu0.trai_ty().into(),
            ),
            explicit_covariant_ty0_to_ty0: ValidTermCurry::new(
                db,
                ValidTermCurryKind::Explicit,
                Variance::Covariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_contravariant_ty0_to_ty0: ValidTermCurry::new(
                db,
                ValidTermCurryKind::Explicit,
                Variance::Contravariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_invariant_ty0_to_ty0: ValidTermCurry::new(
                db,
                ValidTermCurryKind::Explicit,
                Variance::Invariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            parent: menu0,
        }
    }

    pub fn eval_ref_ty(&self) -> ValidTerm {
        self.eval_ref_ty
    }

    pub fn static_ref_ty(&self) -> ValidTerm {
        self.static_ref_ty
    }

    pub fn invariant_ty0_to_trai_ty(&self) -> ValidTermCurry {
        self.explicit_invariant_ty0_to_trai_ty
    }

    pub fn explicit_covariant_ty0_to_ty0(&self) -> ValidTermCurry {
        self.explicit_covariant_ty0_to_ty0
    }

    pub fn explicit_contravariant_ty0_to_ty0(&self) -> ValidTermCurry {
        self.explicit_contravariant_ty0_to_ty0
    }

    pub fn ex_inv_ty0_to_ty0(&self) -> ValidTermCurry {
        self.explicit_invariant_ty0_to_ty0
    }
}
