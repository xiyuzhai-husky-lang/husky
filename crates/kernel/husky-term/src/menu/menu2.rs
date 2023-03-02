use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu2 {
    static_str_ref: Term,
    ex_co_lifetime_to_ex_co_ty0_to_ty0: TermCurry,
    ex_co_lifetime_to_ex_ct_ty0_to_ty0: TermCurry,
    ex_co_lifetime_to_ex_inv_ty0_to_ty0: TermCurry,
    parent: TermMenu1,
}

impl std::ops::Deref for TermMenu2 {
    type Target = TermMenu1;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu2 {
    pub(crate) fn new(db: &dyn TermDb, toolchain: Toolchain, menu1: TermMenu1) -> TermResult<Self> {
        // db.it_entity_path_term(db.entity_path_menu(toolchain).as_ref()?.r32());
        Ok(TermMenu2 {
            static_str_ref: TermApplication::new(db, menu1.static_ref_ty(), menu1.str_ty_path())
                .into(),
            ex_co_lifetime_to_ex_co_ty0_to_ty0: TermCurry::new(
                db,
                TermCurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.explicit_covariant_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_ct_ty0_to_ty0: TermCurry::new(
                db,
                TermCurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.explicit_contravariant_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_inv_ty0_to_ty0: TermCurry::new(
                db,
                TermCurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.ex_inv_ty0_to_ty0().into(),
            ),
            parent: menu1,
        })
    }

    pub fn static_str_ref(&self) -> Term {
        self.static_str_ref
    }

    pub fn ex_co_lifetime_to_ex_co_ty0_to_ty0(&self) -> TermCurry {
        self.ex_co_lifetime_to_ex_co_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_ct_ty0_to_ty0(&self) -> TermCurry {
        self.ex_co_lifetime_to_ex_ct_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_inv_ty0_to_ty0(&self) -> TermCurry {
        self.ex_co_lifetime_to_ex_inv_ty0_to_ty0
    }
}
