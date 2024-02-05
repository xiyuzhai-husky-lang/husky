use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub struct EthRitchieRegularParameter {
    contract: TermContract,
    ty: EthTerm,
}

impl EthRitchieRegularParameter {
    pub fn from_dec(
        db: &::salsa::Db,
        param: DeclarativeRitchieRegularParameter,
    ) -> EthTermResult<Self> {
        Ok(EthRitchieRegularParameter {
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

    #[inline(never)]
    pub(super) fn display_fmt_with_db_and_ctx(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        // ad hoc
        f.write_str(self.contract.as_str())?;
        f.write_str(" ")?;
        self.ty.display_fmt_with_db_and_ctx(f, db, ctx)
    }
}

impl EthInstantiate for EthRitchieRegularParameter {
    type Output = Self;

    fn instantiate(self, db: &::salsa::Db, instantiation: &EthInstantiation) -> Self {
        Self {
            contract: self.contract,
            ty: self.ty.instantiate(db, instantiation),
        }
    }
}

impl salsa::DisplayWithDb for EthRitchieRegularParameter {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.ty
            .display_fmt_with_db_and_ctx(f, db, &mut Default::default())
    }
}

impl EthRitchieRegularParameter {
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
