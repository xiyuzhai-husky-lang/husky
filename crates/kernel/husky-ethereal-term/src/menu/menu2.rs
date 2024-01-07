use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu2 {
    static_str_ref: EtherealTerm,
    ex_co_lifetime_to_ex_co_ty0_to_ty0: EtherealTermCurry,
    ex_co_lifetime_to_ex_ct_ty0_to_ty0: EtherealTermCurry,
    ex_co_lifetime_to_ex_inv_ty0_to_ty0: EtherealTermCurry,
    parent: TermMenu1,
}

impl std::ops::Deref for TermMenu2 {
    type Target = TermMenu1;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu2 {
    pub(crate) fn new(db: &::salsa::Db, _toolchain: Toolchain, menu1: TermMenu1) -> Self {
        TermMenu2 {
            static_str_ref: EtherealTermApplication::new(
                db,
                menu1.static_ref_ty(),
                menu1.str_ty_ontology(),
            )
            .expect("valid toolchain")
            .into(),
            ex_co_lifetime_to_ex_co_ty0_to_ty0: EtherealTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.ex_co_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_ct_ty0_to_ty0: EtherealTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.explicit_contravariant_ty0_to_ty0().into(),
            ),
            ex_co_lifetime_to_ex_inv_ty0_to_ty0: EtherealTermCurry::new(
                db,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu1.lifetime_ty().into(),
                menu1.ex_inv_ty0_to_ty0().into(),
            ),
            parent: menu1,
        }
    }

    pub fn static_str_ref(&self) -> EtherealTerm {
        self.static_str_ref
    }

    pub fn ex_co_lifetime_to_ex_co_ty0_to_ty0(&self) -> EtherealTermCurry {
        self.ex_co_lifetime_to_ex_co_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_ct_ty0_to_ty0(&self) -> EtherealTermCurry {
        self.ex_co_lifetime_to_ex_ct_ty0_to_ty0
    }

    pub fn ex_co_lifetime_to_ex_inv_ty0_to_ty0(&self) -> EtherealTermCurry {
        self.ex_co_lifetime_to_ex_inv_ty0_to_ty0
    }
}
