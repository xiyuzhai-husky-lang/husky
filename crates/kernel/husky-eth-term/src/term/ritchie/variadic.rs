use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub struct EtherealRitchieVariadicParameter {
    contract: TermContract,
    ty: EthTerm,
}

impl EtherealRitchieVariadicParameter {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        param: DeclarativeRitchieVariadicParameter,
    ) -> EthTermResult<Self> {
        Ok(EtherealRitchieVariadicParameter {
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
    pub(super) fn display_fmt_with_db_and_ctx(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        // todo!();
        self.ty.display_fmt_with_db_and_ctx(f, db, ctx)
    }
}

impl salsa::DisplayWithDb for EtherealRitchieVariadicParameter {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.ty
            .display_fmt_with_db_and_ctx(f, db, &mut Default::default())
    }
}

impl EtherealRitchieVariadicParameter {
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
