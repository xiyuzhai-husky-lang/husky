use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DecTermMenu2 {
    static_str_ref: DecTerm,
    ex_co_lifetime_to_ex_co_ty0_to_ty0: DecCurry,
    ex_co_lifetime_to_ex_ct_ty0_to_ty0: DecCurry,
    ex_co_lifetime_to_ex_inv_ty0_to_ty0: DecCurry,
    parent: DecTermMenu1,
}

impl std::ops::Deref for DecTermMenu2 {
    type Target = DecTermMenu1;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DecTermMenu2 {
    pub(crate) fn new(
        db: &::salsa::Db,
        toolchain: Toolchain,
        menu1: DecTermMenu1,
    ) -> DecTermResult<Self> {
        // db.it_item_path_term(item_path_menu(db,toolchain).as_ref()?.r32());
        Ok(DecTermMenu2 {
            static_str_ref: DecApplication::new(db, menu1.static_ref_ty(), menu1.str_ty_path())
                .into(),
            ex_co_lifetime_to_ex_co_ty0_to_ty0: DecCurry::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Covariant,
                menu1.lifetime_ty().into(),
                menu1.explicit_covariant_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_ct_ty0_to_ty0: DecCurry::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Covariant,
                menu1.lifetime_ty().into(),
                menu1.explicit_contravariant_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_inv_ty0_to_ty0: DecCurry::new_nondependent(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Covariant,
                menu1.lifetime_ty().into(),
                menu1.ex_inv_ty0_to_ty0().into(),
            ),
            parent: menu1,
        })
    }

    pub fn static_str_ref(&self) -> DecTerm {
        self.static_str_ref
    }

    pub fn ex_co_lifetime_to_ex_co_ty0_to_ty0(&self) -> DecCurry {
        self.ex_co_lifetime_to_ex_co_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_ct_ty0_to_ty0(&self) -> DecCurry {
        self.ex_co_lifetime_to_ex_ct_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_inv_ty0_to_ty0(&self) -> DecCurry {
        self.ex_co_lifetime_to_ex_inv_ty0_to_ty0
    }
}
