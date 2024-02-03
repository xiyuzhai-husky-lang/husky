use self::term::{application::EthApplication, curry::EthCurry};
use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu1 {
    parent: TermMenu0,
    static_ref_ty: EthTerm,
    explicit_invariant_ty0_to_trai_ty: EthCurry,
    explicit_covariant_ty0_to_ty0: EthCurry,
    explicit_contravariant_ty0_to_ty0: EthCurry,
    explicit_invariant_ty0_to_ty0: EthCurry,
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
            static_ref_ty: EthApplication::new(
                db,
                menu0.ref_ty_path().into(),
                menu0.static_lifetime().into(),
            )
            .unwrap()
            .into(),
            explicit_invariant_ty0_to_trai_ty: EthCurry::new(
                toolchain,
                CurryKind::Explicit,
                Variance::Invariant,
                None,
                menu0.ty0().into(),
                menu0.trai_ty_ontology().into(),
                db,
            ),
            explicit_covariant_ty0_to_ty0: EthCurry::new(
                toolchain,
                CurryKind::Explicit,
                Variance::Covariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
                db,
            ),
            explicit_contravariant_ty0_to_ty0: EthCurry::new(
                toolchain,
                CurryKind::Explicit,
                Variance::Contravariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
                db,
            ),
            explicit_invariant_ty0_to_ty0: EthCurry::new(
                toolchain,
                CurryKind::Explicit,
                Variance::Invariant,
                None,
                menu0.ty0().into(),
                menu0.ty0().into(),
                db,
            ),
            parent: menu0,
        }
    }

    pub fn static_ref_ty(&self) -> EthTerm {
        self.static_ref_ty
    }

    pub fn ex_inv_ty0_to_trai_ty(&self) -> EthCurry {
        self.explicit_invariant_ty0_to_trai_ty
    }

    pub fn ex_co_ty0_to_ty0(&self) -> EthCurry {
        self.explicit_covariant_ty0_to_ty0
    }

    pub fn explicit_contravariant_ty0_to_ty0(&self) -> EthCurry {
        self.explicit_contravariant_ty0_to_ty0
    }

    pub fn ex_inv_ty0_to_ty0(&self) -> EthCurry {
        self.explicit_invariant_ty0_to_ty0
    }
}
