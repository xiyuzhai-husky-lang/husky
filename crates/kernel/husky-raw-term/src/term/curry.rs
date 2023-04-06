pub use context::*;

use crate::*;

/// representing raw_term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermCurry {
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub parameter_variable: Option<RawTermPlaceholder>,
    /// X
    pub parameter_ty: RawTerm,
    /// Y
    pub return_ty: RawTerm,
}

impl RawTermCurry {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
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

#[salsa::tracked(jar = RawTermJar)]
pub(crate) fn total_number_of_curry_parameters(db: &dyn RawTermDb, term: RawTermCurry) -> u8 {
    term.return_ty(db).total_number_of_curry_parameters(db) + 1
}

impl<Db: RawTermDb + ?Sized> salsa::DisplayWithDb<Db> for RawTermCurry {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<RawTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl RawTermRewriteCopy for RawTermCurry {
    fn substitute(self, _db: &dyn RawTermDb, _substituation: &RawTermSubstitution) -> Self {
        todo!()
    }
}
