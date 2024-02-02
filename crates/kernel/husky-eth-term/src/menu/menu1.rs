use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu1 {
    parent: TermMenu0,
    static_ref_ty: EthTerm,
    explicit_invariant_ty0_to_trai_ty: CurryEthTerm,
    explicit_covariant_ty0_to_ty0: CurryEthTerm,
    explicit_contravariant_ty0_to_ty0: CurryEthTerm,
    explicit_invariant_ty0_to_ty0: CurryEthTerm,
}

impl std::ops::Deref for TermMenu1 {
    type Target = TermMenu0;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu1 {
    pub fn new(db: &::salsa::Db, toolchain: Toolchain, menu0: TermMenu0) -> Self {
        // todo!()
        Self {
            static_ref_ty: ApplicationEthTerm::new(
                db,
                menu0.ref_ty_path().into(),
                menu0.static_lifetime().into(),
            )
            .unwrap()
            .into(),
            explicit_invariant_ty0_to_trai_ty: CurryEthTerm::new(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Invariant,
                None,
                menu0.ty0().into(),
                menu0.trai_ty_ontology().into(),
            ),
            explicit_covariant_ty0_to_ty0: CurryEthTerm::new(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_contravariant_ty0_to_ty0: CurryEthTerm::new(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Contravariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            explicit_invariant_ty0_to_ty0: CurryEthTerm::new(
                db,
                toolchain,
                CurryKind::Explicit,
                Variance::Invariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
            ),
            parent: menu0,
        }
    }

    pub fn static_ref_ty(&self) -> EthTerm {
        self.static_ref_ty
    }

    pub fn ex_inv_ty0_to_trai_ty(&self) -> CurryEthTerm {
        self.explicit_invariant_ty0_to_trai_ty
    }

    pub fn ex_co_ty0_to_ty0(&self) -> CurryEthTerm {
        self.explicit_covariant_ty0_to_ty0
    }

    pub fn explicit_contravariant_ty0_to_ty0(&self) -> CurryEthTerm {
        self.explicit_contravariant_ty0_to_ty0
    }

    pub fn ex_inv_ty0_to_ty0(&self) -> CurryEthTerm {
        self.explicit_invariant_ty0_to_ty0
    }
}
