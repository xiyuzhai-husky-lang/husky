use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub struct EtherealRitchieRegularParameter {
    contract: TermContract,
    ty: EthTerm,
}

impl EtherealRitchieRegularParameter {
    pub fn from_declarative(
        db: &::salsa::Db,
        param: DeclarativeRitchieRegularParameter,
    ) -> EthTermResult<Self> {
        Ok(EtherealRitchieRegularParameter {
            contract: param.contract(),
            ty: EthTerm::ty_from_declarative(db, param.ty())?,
        })
    }

    pub(super) fn reduce(self, db: &::salsa::Db) -> Self {
        Self {
            contract: self.contract,
            ty: self.ty.reduce(db),
        }
    }

    #[inline(never)]
    pub(super) fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        // ad hoc
        f.write_str(self.contract.as_str())?;
        f.write_str(" ")?;
        self.ty.show_with_db_fmt(f, db, ctx)
    }
}

impl EthTermInstantiate for EtherealRitchieRegularParameter {
    type Output = Self;

    fn instantiate(self, db: &::salsa::Db, instantiation: &EtherealInstantiation) -> Self {
        Self {
            contract: self.contract,
            ty: self.ty.instantiate(db, instantiation),
        }
    }
}

impl salsa::DisplayWithDb for EtherealRitchieRegularParameter {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.ty.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl EtherealRitchieRegularParameter {
    pub fn new(contract: TermContract, ty: EthTerm) -> Self {
        Self { contract, ty }
    }

    pub fn contract(&self) -> TermContract {
        self.contract
    }

    pub fn ty(&self) -> EthTerm {
        self.ty
    }
}
