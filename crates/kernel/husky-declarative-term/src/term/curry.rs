pub use context::*;

use crate::*;

/// representing declarative_term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar, constructor = new_inner)]
pub struct DeclarativeTermCurry {
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub parameter_variable: Option<DeclarativeTermVariable>,
    /// X
    pub parameter_ty: DeclarativeTerm,
    /// Y
    pub return_ty: DeclarativeTerm,
}

impl DeclarativeTermCurry {
    pub fn new_dependent(
        db: &dyn DeclarativeTermDb,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_symbol: DeclarativeTermSymbol,
        parameter_ty: DeclarativeTerm,
        return_ty: DeclarativeTerm,
    ) -> Self {
        let (return_ty, parameter_variable) = return_ty.r#abstract(db, parameter_symbol);
        DeclarativeTermCurry::new_inner(
            db,
            curry_kind,
            variance,
            parameter_variable,
            parameter_ty,
            return_ty,
        )
    }

    pub fn new_nondependent(
        db: &dyn DeclarativeTermDb,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_ty: DeclarativeTerm,
        return_ty: DeclarativeTerm,
    ) -> Self {
        DeclarativeTermCurry::new_inner(db, curry_kind, variance, None, parameter_ty, return_ty)
    }

    pub(super) fn substitute_symbol_with_variable(
        self,
        db: &dyn DeclarativeTermDb,
        symbol: DeclarativeTermSymbol,
        variable: DeclarativeTermVariable,
    ) -> Self {
        DeclarativeTermCurry::new_inner(
            db,
            self.curry_kind(db),
            self.variance(db),
            self.parameter_variable(db),
            self.parameter_ty(db)
                .substitute_symbol_with_variable(db, symbol, variable),
            self.return_ty(db)
                .substitute_symbol_with_variable(db, symbol, variable),
        )
    }

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
