use salsa::Database;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
// #[salsa::derive_debug_with_db(db = EtherealTermDb)]
pub struct EtherealRitchieKeyedParameter {
    key: Ident,
    contract: Contract,
    ty: EtherealTerm,
    has_default: bool,
}

impl EtherealRitchieKeyedParameter {
    pub fn new(key: Ident, contract: Contract, ty: EtherealTerm, has_default: bool) -> Self {
        Self {
            key,
            contract,
            ty,
            has_default,
        }
    }

    pub(super) fn from_declarative(
        db: &dyn EtherealTermDb,
        param: DeclarativeRitchieKeyedParameter,
    ) -> EtherealTermResult<Self> {
        let ty = EtherealTerm::ty_from_declarative(db, param.ty())?;
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

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }

    pub fn has_default(&self) -> bool {
        self.has_default
    }

    pub(super) fn reduce(self, db: &dyn EtherealTermDb) -> Self {
        Self {
            key: self.key,
            contract: self.contract,
            ty: self.ty.reduce(db),
            has_default: self.has_default,
        }
    }

    #[inline(never)]
    pub(super) fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EtherealTermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        // todo!();
        self.ty.show_with_db_fmt(f, db, ctx)
    }
}

impl salsa::DisplayWithDb for EtherealRitchieKeyedParameter {
    fn display_with_db_fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &Db) -> std::fmt::Result {
        self.ty.show_with_db_fmt(f, db(), &mut Default::default())
    }
}
