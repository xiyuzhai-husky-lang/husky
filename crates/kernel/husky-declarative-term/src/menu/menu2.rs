use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclarativeTermMenu2 {
    static_str_ref: DeclarativeTerm,
    ex_co_lifetime_to_ex_co_ty0_to_ty0: DeclarativeTermCurry,
    ex_co_lifetime_to_ex_ct_ty0_to_ty0: DeclarativeTermCurry,
    ex_co_lifetime_to_ex_inv_ty0_to_ty0: DeclarativeTermCurry,
    parent: DeclarativeTermMenu1,
}

impl std::ops::Deref for DeclarativeTermMenu2 {
    type Target = DeclarativeTermMenu1;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DeclarativeTermMenu2 {
    pub(crate) fn new(
        db: &dyn DeclarativeTermDb,
        _toolchain: Toolchain,
        menu1: DeclarativeTermMenu1,
    ) -> DeclarativeTermResult<Self> {
        // db.it_entity_path_term(db.entity_path_menu(toolchain).as_ref()?.r32());
        Ok(DeclarativeTermMenu2 {
            static_str_ref: DeclarativeTermExplicitApplication::new(
                db,
                menu1.static_ref_ty(),
                menu1.str_ty_path(),
            )
            .into(),
            ex_co_lifetime_to_ex_co_ty0_to_ty0: DeclarativeTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.explicit_covariant_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_ct_ty0_to_ty0: DeclarativeTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.explicit_contravariant_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_inv_ty0_to_ty0: DeclarativeTermCurry::new(
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

    pub fn static_str_ref(&self) -> DeclarativeTerm {
        self.static_str_ref
    }

    pub fn ex_co_lifetime_to_ex_co_ty0_to_ty0(&self) -> DeclarativeTermCurry {
        self.ex_co_lifetime_to_ex_co_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_ct_ty0_to_ty0(&self) -> DeclarativeTermCurry {
        self.ex_co_lifetime_to_ex_ct_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_inv_ty0_to_ty0(&self) -> DeclarativeTermCurry {
        self.ex_co_lifetime_to_ex_inv_ty0_to_ty0
    }
}
