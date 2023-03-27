pub use context::*;

use super::*;

/// representing term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = TermDb, jar = TermJar, constructor = pub(crate) new)]
pub struct TermCurry {
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub parameter_variable: Option<TermVariable>,
    /// X
    pub parameter_ty: Term,
    /// Y
    pub return_ty: Term,
}

#[test]
fn term_curry_size_works() {
    assert_eq!(std::mem::size_of::<TermCurry>(), std::mem::size_of::<u32>());
}

impl TermCurry {
    pub(crate) fn from_raw_unchecked(
        db: &dyn TermDb,
        raw_term_curry: RawTermCurry,
    ) -> TermResult<Self> {
        term_curry_from_raw_unchecked(db, raw_term_curry)
    }

    pub(super) fn check(self, db: &dyn TermDb) -> TermResult<()> {
        match self.parameter_ty(db).raw_ty(db)? {
            Left(RawTerm::Category(_)) => (),
            _ => todo!(),
        };
        match self.return_ty(db).raw_ty(db)? {
            Left(RawTerm::Category(_)) => Ok(()),
            _ => todo!(),
        }
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
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

    pub fn substitute(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Term {
        todo!()
    }
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn term_curry_from_raw_unchecked(
    db: &dyn TermDb,
    raw_term_curry: RawTermCurry,
) -> TermResult<TermCurry> {
    let t =
        |raw_ty| Term::from_raw_unchecked(db, raw_ty, TermTypeExpectation::FinalDestinationEqsSort);
    Ok(TermCurry::new(
        db,
        raw_term_curry.curry_kind(db),
        raw_term_curry.variance(db),
        match raw_term_curry.parameter_variable(db) {
            Some(parameter_variable) => {
                Some(TermVariable::from_raw_unchecked(db, parameter_variable)?)
            }
            None => None,
        },
        t(raw_term_curry.parameter_ty(db))?,
        t(raw_term_curry.return_ty(db))?,
    ))
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn check_term_curry_validity(db: &dyn TermDb, term_curry: TermCurry) -> TermResult<()> {
    todo!()
}

impl<Db: TermDb + ?Sized> salsa::DisplayWithDb<Db> for TermCurry {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}
