pub use context::*;

use super::*;

/// representing term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar, constructor = pub(crate) new)]
pub struct EtherealTermCurry {
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub parameter_variable: Option<EtherealTermPlaceholder>,
    /// X
    pub parameter_ty: EtherealTerm,
    /// Y
    pub return_ty: EtherealTerm,
}

#[test]
fn term_curry_size_works() {
    assert_eq!(
        std::mem::size_of::<EtherealTermCurry>(),
        std::mem::size_of::<u32>()
    );
}

impl EtherealTermCurry {
    pub(crate) fn from_raw_unchecked(
        db: &dyn EtherealTermDb,
        raw_term_curry: RawTermCurry,
    ) -> TermResult<Self> {
        term_curry_from_raw_unchecked(db, raw_term_curry)
    }

    pub(super) fn check(self, db: &dyn EtherealTermDb) -> TermResult<()> {
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
        db: &dyn EtherealTermDb,
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

    pub fn substitute(
        self,
        db: &dyn EtherealTermDb,
        substituation: &TermSubstitution,
    ) -> EtherealTerm {
        todo!()
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn term_curry_from_raw_unchecked(
    db: &dyn EtherealTermDb,
    raw_term_curry: RawTermCurry,
) -> TermResult<EtherealTermCurry> {
    let t = |raw_ty| {
        EtherealTerm::from_raw_unchecked(db, raw_ty, TermTypeExpectation::FinalDestinationEqsSort)
    };
    Ok(EtherealTermCurry::new(
        db,
        raw_term_curry.curry_kind(db),
        raw_term_curry.variance(db),
        match raw_term_curry.parameter_variable(db) {
            Some(parameter_variable) => Some(EtherealTermPlaceholder::from_raw_unchecked(
                db,
                parameter_variable,
            )?),
            None => None,
        },
        t(raw_term_curry.parameter_ty(db))?,
        t(raw_term_curry.return_ty(db))?,
    ))
}

impl<Db: EtherealTermDb + ?Sized> salsa::DisplayWithDb<Db> for EtherealTermCurry {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EtherealTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}
