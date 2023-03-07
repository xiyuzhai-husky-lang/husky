use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct RawTermMenu2 {
    static_str_ref: RawTerm,
    ex_co_lifetime_to_ex_co_ty0_to_ty0: RawTermCurry,
    ex_co_lifetime_to_ex_ct_ty0_to_ty0: RawTermCurry,
    ex_co_lifetime_to_ex_inv_ty0_to_ty0: RawTermCurry,
    parent: RawTermMenu1,
}

impl std::ops::Deref for RawTermMenu2 {
    type Target = RawTermMenu1;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl RawTermMenu2 {
    pub(crate) fn new(
        db: &dyn RawTermDb,
        toolchain: Toolchain,
        menu1: RawTermMenu1,
    ) -> RawTermResult<Self> {
        // db.it_entity_path_term(db.entity_path_menu(toolchain).as_ref()?.r32());
        Ok(RawTermMenu2 {
            static_str_ref: RawTermExplicitApplication::new(
                db,
                menu1.static_ref_ty(),
                menu1.str_ty_path(),
            )
            .into(),
            ex_co_lifetime_to_ex_co_ty0_to_ty0: RawTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.explicit_covariant_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_ct_ty0_to_ty0: RawTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.explicit_contravariant_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_inv_ty0_to_ty0: RawTermCurry::new(
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

    pub fn static_str_ref(&self) -> RawTerm {
        self.static_str_ref
    }

    pub fn ex_co_lifetime_to_ex_co_ty0_to_ty0(&self) -> RawTermCurry {
        self.ex_co_lifetime_to_ex_co_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_ct_ty0_to_ty0(&self) -> RawTermCurry {
        self.ex_co_lifetime_to_ex_ct_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_inv_ty0_to_ty0(&self) -> RawTermCurry {
        self.ex_co_lifetime_to_ex_inv_ty0_to_ty0
    }
}
