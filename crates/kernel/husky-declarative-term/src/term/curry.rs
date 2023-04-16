pub use context::*;

use crate::*;

/// representing raw_term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermCurry {
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub parameter_variable: Option<DeclarativeTermPlaceholder>,
    /// X
    pub parameter_ty: DeclarativeTerm,
    /// Y
    pub return_ty: DeclarativeTerm,
}

impl DeclarativeTermCurry {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn DeclarativeTermDb,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        let parameter_variable = self.parameter_variable(db);
        if parameter_variable.is_some() {
            f.write_str("(")?
        }
        f.write_str(self.variance(db).as_str())?;
        if let Some(parameter_variable) = parameter_variable {
            ctx.fmt_with_variable(db, parameter_variable, |ctx| {
                ctx.fmt_variable(db, parameter_variable, f);
                f.write_str(": ")?;
                self.parameter_ty(db).show_with_db_fmt(f, db, ctx)?;
                f.write_str(") -> ")?;
                self.return_ty(db).show_with_db_fmt(f, db, ctx)
            })
        } else {
            self.parameter_ty(db).show_with_db_fmt(f, db, ctx)?;
            f.write_str(" -> ")?;
            self.return_ty(db).show_with_db_fmt(f, db, ctx)
        }
    }
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn total_number_of_curry_parameters(
    db: &dyn DeclarativeTermDb,
    term: DeclarativeTermCurry,
) -> u8 {
    term.return_ty(db).total_number_of_curry_parameters(db) + 1
}

impl<Db: DeclarativeTermDb + ?Sized> salsa::DisplayWithDb<Db> for DeclarativeTermCurry {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclarativeTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl DeclarativeTermRewriteCopy for DeclarativeTermCurry {
    fn substitute(
        self,
        _db: &dyn DeclarativeTermDb,
        _substituation: &DeclarativeTermSubstitution,
    ) -> Self {
        todo!()
    }
}
