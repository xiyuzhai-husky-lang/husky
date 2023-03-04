use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ValidTermMenu2 {
    static_str_ref: ValidTerm,
    ex_co_lifetime_to_ex_co_ty0_to_ty0: ValidTermCurry,
    ex_co_lifetime_to_ex_ct_ty0_to_ty0: ValidTermCurry,
    ex_co_lifetime_to_ex_inv_ty0_to_ty0: ValidTermCurry,
    parent: ValidTermMenu1,
}

impl std::ops::Deref for ValidTermMenu2 {
    type Target = ValidTermMenu1;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl ValidTermMenu2 {
    pub(crate) fn new(
        db: &dyn ValidTermDb,
        toolchain: Toolchain,
        menu1: ValidTermMenu1,
    ) -> ValidTermResult<Self> {
        // db.it_entity_path_valid_term(db.entity_path_menu(toolchain).as_ref()?.r32());
        Ok(ValidTermMenu2 {
            static_str_ref: ValidTermApplication::new(
                db,
                menu1.static_ref_ty(),
                menu1.str_ty_path(),
                0,
            )
            .unwrap()
            .into(),
            ex_co_lifetime_to_ex_co_ty0_to_ty0: ValidTermCurry::new(
                db,
                ValidCurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.explicit_covariant_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_ct_ty0_to_ty0: ValidTermCurry::new(
                db,
                ValidCurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.explicit_contravariant_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_inv_ty0_to_ty0: ValidTermCurry::new(
                db,
                ValidCurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.ex_inv_ty0_to_ty0().into(),
            ),
            parent: menu1,
        })
    }

    pub fn static_str_ref(&self) -> ValidTerm {
        self.static_str_ref
    }

    pub fn ex_co_lifetime_to_ex_co_ty0_to_ty0(&self) -> ValidTermCurry {
        self.ex_co_lifetime_to_ex_co_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_ct_ty0_to_ty0(&self) -> ValidTermCurry {
        self.ex_co_lifetime_to_ex_ct_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_inv_ty0_to_ty0(&self) -> ValidTermCurry {
        self.ex_co_lifetime_to_ex_inv_ty0_to_ty0
    }
}
