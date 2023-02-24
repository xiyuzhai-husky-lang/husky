use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu2 {
    static_str_ref: Term,
    covariant_lifetime_to_covariant_ty0_to_ty0: TermCurry,
    covariant_lifetime_to_contravariant_ty0_to_ty0: TermCurry,
    covariant_lifetime_to_invariant_ty0_to_ty0: TermCurry,
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
            covariant_lifetime_to_covariant_ty0_to_ty0: TermCurry::new(
                db,
                Variance::Covariant,
                menu1.lifetime_ty().into(),
                menu1.covariant_ty0_to_ty0().into(),
            ),
            covariant_lifetime_to_contravariant_ty0_to_ty0: TermCurry::new(
                db,
                Variance::Covariant,
                menu1.lifetime_ty().into(),
                menu1.contravariant_ty0_to_ty0().into(),
            ),
            covariant_lifetime_to_invariant_ty0_to_ty0: TermCurry::new(
                db,
                Variance::Covariant,
                menu1.lifetime_ty().into(),
                menu1.invariant_ty0_to_ty0().into(),
            ),
            parent: menu1,
        })
    }

    pub fn static_str_ref(&self) -> Term {
        self.static_str_ref
    }

    pub fn covariant_lifetime_to_covariant_ty0_to_ty0(&self) -> TermCurry {
        self.covariant_lifetime_to_covariant_ty0_to_ty0
    }

    pub fn covariant_lifetime_to_contravariant_ty0_to_ty0(&self) -> TermCurry {
        self.covariant_lifetime_to_contravariant_ty0_to_ty0
    }

    pub fn covariant_lifetime_to_invariant_ty0_to_ty0(&self) -> TermCurry {
        self.covariant_lifetime_to_invariant_ty0_to_ty0
    }
}
