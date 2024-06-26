use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
pub struct EtherealRitchieVariadicParameter {
    contract: Contract,
    ty: EthTerm,
}

impl EtherealRitchieVariadicParameter {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        param: DeclarativeRitchieVariadicParameter,
    ) -> EthTermResult<Self> {
        Ok(EtherealRitchieVariadicParameter {
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

impl salsa::DisplayWithDb for EtherealRitchieVariadicParameter {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.ty.display_fmt_with_db(f, db)
    }
}

impl EtherealRitchieVariadicParameter {
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
