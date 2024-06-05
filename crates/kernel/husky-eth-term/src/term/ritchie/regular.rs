use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
pub struct EthRitchieSimpleParameter {
    contract: Contract,
    ty: EthTerm,
}

impl EthRitchieSimpleParameter {
    pub fn from_dec(
        db: &::salsa::Db,
        param: DeclarativeRitchieSimpleParameter,
    ) -> EthTermResult<Self> {
        Ok(EthRitchieSimpleParameter {
            contract: param.contract(),
            ty: EthTerm::ty_from_dec(db, param.ty())?,
        })
    }

    pub(super) fn reduce(self, db: &::salsa::Db) -> Self {
        Self {
            contract: self.contract,
            ty: self.ty.reduce(db),
        }
    }
}

impl EthInstantiate for EthRitchieSimpleParameter {
    type Output = Self;

    fn instantiate(
        self,
        instantiation: &EthInstantiation,
        ctx: &impl IsEthInstantiationContext,
        db: &::salsa::Db,
    ) -> Self {
        Self {
            contract: self.contract,
            ty: self.ty.instantiate(instantiation, ctx, db),
        }
    }
}

impl salsa::DisplayWithDb for EthRitchieSimpleParameter {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        // ad hoc
        f.write_str(self.contract.as_str())?;
        f.write_str(" ")?;
        self.ty.display_fmt_with_db(f, db)
    }
}

impl EthRitchieSimpleParameter {
    pub fn new(contract: Contract, ty: EthTerm) -> Self {
        Self { contract, ty }
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> EthTerm {
        self.ty
    }
}
