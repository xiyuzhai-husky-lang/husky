use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
// #[salsa::derive_debug_with_db(db = EthTermDb)]
pub struct EtherealRitchieKeyedParameter {
    key: Ident,
    contract: TermContract,
    ty: EthTerm,
    has_default: bool,
}

impl EtherealRitchieKeyedParameter {
    pub fn new(key: Ident, contract: TermContract, ty: EthTerm, has_default: bool) -> Self {
        Self {
            key,
            contract,
            ty,
            has_default,
        }
    }

    pub(super) fn from_dec(
        db: &::salsa::Db,
        param: DeclarativeRitchieKeyedParameter,
    ) -> EthTermResult<Self> {
        let ty = EthTerm::ty_from_dec(db, param.ty())?;
        let has_default = param.has_default();
        Ok(EtherealRitchieKeyedParameter {
            key: param.key(),
            contract: param.contract(),
            ty,
            has_default,
        })
    }

    pub fn key(&self) -> Ident {
        self.key
    }

    pub fn contract(&self) -> TermContract {
        self.contract
    }

    pub fn ty(&self) -> EthTerm {
        self.ty
    }

    pub fn has_default(&self) -> bool {
        self.has_default
    }

    pub(super) fn reduce(self, db: &::salsa::Db) -> Self {
        Self {
            key: self.key,
            contract: self.contract,
            ty: self.ty.reduce(db),
            has_default: self.has_default,
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

impl salsa::DisplayWithDb for EtherealRitchieKeyedParameter {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.ty
            .display_fmt_with_db_and_ctx(f, db, &mut Default::default())
    }
}
