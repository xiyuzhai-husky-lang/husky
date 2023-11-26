use salsa::Database;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct EtherealRitchieVariadicParameter {
    contract: Contract,
    ty: EtherealTerm,
}

impl EtherealRitchieVariadicParameter {
    pub(super) fn from_declarative(
        db: &dyn EtherealTermDb,
        param: DeclarativeRitchieVariadicParameter,
    ) -> EtherealTermResult<Self> {
        Ok(EtherealRitchieVariadicParameter {
            contract: param.contract(),
            ty: EtherealTerm::ty_from_declarative(db, param.ty())?,
        })
    }

    pub(super) fn reduce(self, db: &dyn EtherealTermDb) -> Self {
        Self {
            contract: self.contract,
            ty: self.ty.reduce(db),
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

impl salsa::DisplayWithDb for EtherealRitchieVariadicParameter {
    fn display_with_db_fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &Db) -> std::fmt::Result {
        self.ty.show_with_db_fmt(
            f,
            db.as_jar_db_dyn::<EtherealTermJar>(),
            &mut Default::default(),
        )
    }
}

impl EtherealRitchieVariadicParameter {
    pub fn new(contract: Contract, ty: EtherealTerm) -> Self {
        Self { contract, ty }
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }
}
