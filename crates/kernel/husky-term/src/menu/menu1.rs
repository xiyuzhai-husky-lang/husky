use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu1 {
    parent: TermMenu0,
    eval_ref_ty: Term,
    static_ref_ty: Term,
    invariant_ty0_to_trai_ty: TermCurry,
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
            .into(),
            static_ref_ty: TermApplication::new(db, menu0.ref_ty_path(), menu0.static_lifetime())
                .into(),
            invariant_ty0_to_trai_ty: TermCurry::new(
                db,
                Variance::Invariant,
                menu0.ty0().into(),
                menu0.trai_ty().into(),
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

    pub fn invariant_ty0_to_trai_ty(&self) -> TermCurry {
        self.invariant_ty0_to_trai_ty
    }
}
