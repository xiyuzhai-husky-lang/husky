use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EtherealTermDb)]
pub struct EtherealTermRitchieKeyedParameter {
    key: Ident,
    contract: Contract,
    ty: EtherealTerm,
    default: Option<EtherealTerm>,
}

impl EtherealTermRitchieKeyedParameter {
    pub(super) fn from_declarative(
        db: &dyn EtherealTermDb,
        param: &DeclarativeTermRitchieKeyedParameter,
    ) -> EtherealTermResult<Self> {
        let ty = EtherealTerm::ty_from_declarative(db, param.ty())?;
        let default = match param.default() {
            Some(default) => Some(EtherealTerm::from_declarative(
                db,
                default,
                ty.ty_expectation(db)?,
            )?),
            None => None,
        };
        Ok(EtherealTermRitchieKeyedParameter {
            key: param.key(),
            contract: param.contract(),
            ty,
            default,
        })
    }

    pub(super) fn reduce(self, db: &dyn EtherealTermDb) -> Self {
        Self {
            key: self.key,
            contract: self.contract,
            ty: self.ty.reduce(db),
            default: self.default.map(|default| default.reduce(db)),
        }
    }

    pub(super) fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EtherealTermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        todo!();
        self.ty.show_with_db_fmt(f, db, ctx)
    }
}

impl<Db> salsa::DisplayWithDb<Db> for EtherealTermRitchieKeyedParameter
where
    Db: EtherealTermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        todo!();
        let db = <Db as salsa::DbWithJar<EtherealTermJar>>::as_jar_db(db);
        self.ty.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl EtherealTermRitchieKeyedParameter {
    pub fn new(
        key: Ident,
        contract: Contract,
        ty: EtherealTerm,
        default: Option<EtherealTerm>,
    ) -> Self {
        Self {
            key,
            contract,
            ty,
            default,
        }
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }
}
