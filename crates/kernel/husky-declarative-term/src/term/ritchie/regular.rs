use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeTermDb)]
pub struct DeclarativeTermRitchieRegularParameter {
    contract: Contract,
    ty: DeclarativeTerm,
}

impl DeclarativeTermRitchieRegularParameter {
    pub fn new(contract: Contract, ty: DeclarativeTerm) -> Self {
        Self { contract, ty }
    }

    pub(crate) fn substitute_ty(self, f: impl FnOnce(DeclarativeTerm) -> DeclarativeTerm) -> Self {
        Self {
            contract: self.contract,
            ty: f(self.ty),
        }
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> DeclarativeTerm {
        self.ty
    }

    pub(super) fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn DeclarativeTermDb,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        self.ty.show_with_db_fmt(f, db, ctx)
    }
}

impl<Db> salsa::DisplayWithDb<Db> for DeclarativeTermRitchieRegularParameter
where
    Db: DeclarativeTermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclarativeTermJar>>::as_jar_db(db);
        self.ty.show_with_db_fmt(f, db, &mut Default::default())
    }
}
