use salsa::Db;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct EtherealRitchieRegularParameter {
    contract: Contract,
    ty: EtherealTerm,
}

impl EtherealRitchieRegularParameter {
    pub fn from_declarative(
        db: &::salsa::Db,
        param: DeclarativeRitchieRegularParameter,
    ) -> EtherealTermResult<Self> {
        Ok(EtherealRitchieRegularParameter {
            contract: param.contract(),
            ty: EtherealTerm::ty_from_declarative(db, param.ty())?,
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

impl EtherealTermInstantiate for EtherealRitchieRegularParameter {
    type Target = Self;

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
