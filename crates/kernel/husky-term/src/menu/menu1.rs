use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu1 {
    parent: TermMenu0,
    eval_ref_ty: Term,
    static_ref_ty: Term,
    explicit_invariant_ty0_to_trai_ty: TermCurry,
    explicit_covariant_ty0_to_ty0: TermCurry,
    explicit_contravariant_ty0_to_ty0: TermCurry,
    explicit_invariant_ty0_to_ty0: TermCurry,
}

impl std::ops::Deref for TermMenu1 {
    type Target = TermMenu0;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu1 {
    pub fn new(db: &dyn TermDb, _toolchain: Toolchain, menu0: TermMenu0) -> Self {
        // todo!()
        Self {
            eval_ref_ty: TermApplication::new(
                db,
                menu0.ref_ty_path().into(),
                menu0.eval_lifetime().into(),
            )
            .unwrap()
            .into(),
            static_ref_ty: TermApplication::new(db, menu0.ref_ty_path(), menu0.static_lifetime())
                .unwrap()
                .into(),
            explicit_invariant_ty0_to_trai_ty: TermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Invariant,
                None,
                menu0.ty0().into(),
                menu0.trai_ty().into(),
            ),
            explicit_covariant_ty0_to_ty0: TermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_contravariant_ty0_to_ty0: TermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Contravariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_invariant_ty0_to_ty0: TermCurry::new(
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

    pub fn eval_ref_ty(&self) -> Term {
        self.eval_ref_ty
    }

    pub fn static_ref_ty(&self) -> Term {
        self.static_ref_ty
    }

    pub fn ex_inv_ty0_to_trai_ty(&self) -> TermCurry {
        self.explicit_invariant_ty0_to_trai_ty
    }

    pub fn ex_co_ty0_to_ty0(&self) -> TermCurry {
        self.explicit_covariant_ty0_to_ty0
    }

    pub fn explicit_contravariant_ty0_to_ty0(&self) -> TermCurry {
        self.explicit_contravariant_ty0_to_ty0
    }

    pub fn ex_inv_ty0_to_ty0(&self) -> TermCurry {
        self.explicit_invariant_ty0_to_ty0
    }
}
