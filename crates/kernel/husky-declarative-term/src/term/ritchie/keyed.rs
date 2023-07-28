use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeTermDb)]
pub struct DeclarativeTermRitchieKeyedParameter {
    key: Ident,
    contract: Contract,
    ty: DeclarativeTerm,
    default: Option<DeclarativeTerm>,
}

impl DeclarativeTermRitchieKeyedParameter {
    pub fn new(
        key: Ident,
        contract: Contract,
        ty: DeclarativeTerm,
        default: Option<DeclarativeTerm>,
    ) -> Self {
        Self {
            key,
            contract,
            ty,
            default,
        }
    }

    pub(crate) fn substitute_ty(self, f: impl Fn(DeclarativeTerm) -> DeclarativeTerm) -> Self {
        Self {
            key: self.key,
            contract: self.contract,
            ty: f(self.ty),
            default: self.default.map(|default| f(default)),
        }
    }

    pub fn key(&self) -> Ident {
        self.key
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> DeclarativeTerm {
        self.ty
    }

    pub fn default(&self) -> Option<DeclarativeTerm> {
        self.default
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

impl<Db> salsa::DisplayWithDb<Db> for DeclarativeTermRitchieKeyedParameter
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
        let mut ctx = Default::default();
        f.write_str(self.key.data(db))?;
        f.write_str(": ")?;
        self.ty.show_with_db_fmt(f, db, &mut ctx)?;
        f.write_str(" = ");
        match self.default {
            Some(default) => default.show_with_db_fmt(f, db, &mut ctx),
            None => f.write_str("_"),
        }
    }
}
