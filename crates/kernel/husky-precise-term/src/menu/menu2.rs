use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct PreciseTermMenu2 {
    static_str_ref: PreciseTerm,
    ex_co_lifetime_to_ex_co_ty0_to_ty0: PreciseTermCurry,
    ex_co_lifetime_to_ex_ct_ty0_to_ty0: PreciseTermCurry,
    ex_co_lifetime_to_ex_inv_ty0_to_ty0: PreciseTermCurry,
    parent: PreciseTermMenu1,
}

impl std::ops::Deref for PreciseTermMenu2 {
    type Target = PreciseTermMenu1;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl PreciseTermMenu2 {
    pub(crate) fn new(
        db: &dyn PreciseTermDb,
        toolchain: Toolchain,
        menu1: PreciseTermMenu1,
    ) -> PreciseTermResult<Self> {
        // db.it_entity_path_precise_term(db.entity_path_menu(toolchain).as_ref()?.r32());
        Ok(PreciseTermMenu2 {
            static_str_ref: PreciseTermApplication::new(
                db,
                menu1.static_ref_ty(),
                menu1.str_ty_path(),
                0,
            )
            .into(),
            ex_co_lifetime_to_ex_co_ty0_to_ty0: PreciseTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.explicit_covariant_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_ct_ty0_to_ty0: PreciseTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.explicit_contravariant_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_inv_ty0_to_ty0: PreciseTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.ex_inv_ty0_to_ty0().into(),
            ),
            parent: menu1,
        })
    }

    pub fn static_str_ref(&self) -> PreciseTerm {
        self.static_str_ref
    }

    pub fn ex_co_lifetime_to_ex_co_ty0_to_ty0(&self) -> PreciseTermCurry {
        self.ex_co_lifetime_to_ex_co_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_ct_ty0_to_ty0(&self) -> PreciseTermCurry {
        self.ex_co_lifetime_to_ex_ct_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_inv_ty0_to_ty0(&self) -> PreciseTermCurry {
        self.ex_co_lifetime_to_ex_inv_ty0_to_ty0
    }
}
