use super::*;
use context::*;

/// representing precise_term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = PreciseTermDb, jar = PreciseTermJar)]
pub struct PreciseTermCurry {
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub input_symbol: Option<PreciseTermSymbol>,
    /// X
    pub input_ty: PreciseTerm,
    /// Y
    pub return_ty: PreciseTerm,
}

impl PreciseTermCurry {
    pub fn from_raw(
        db: &dyn PreciseTermDb,
        raw_term_curry: RawTermCurry,
        raw_ty_expectation: TermTypeExpectation,
    ) -> PreciseTermResult<Self> {
        match raw_ty_expectation {
            TermTypeExpectation::FinalDestinationEqsNonSortTypePath(_) => {
                return Err(PreciseTermError::ExpectationNotMatchedForCurry)
            }
            TermTypeExpectation::FinalDestinationEqsSort | TermTypeExpectation::Any => (),
        }
        precise_term_curry_from_raw(db, raw_term_curry)
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn PreciseTermDb,
        ctx: &mut PreciseTermShowContext,
    ) -> std::fmt::Result {
        let input_symbol = self.input_symbol(db);
        if input_symbol.is_some() {
            f.write_str("(")?
        }
        f.write_str(self.variance(db).as_str())?;
        if let Some(input_symbol) = input_symbol {
            ctx.fmt_with_symbol(db, input_symbol, |ctx| {
                ctx.fmt_symbol(db, input_symbol, f);
                f.write_str(": ")?;
                self.input_ty(db).show_with_db_fmt(f, db, ctx)?;
                f.write_str(") -> ")?;
                self.return_ty(db).show_with_db_fmt(f, db, ctx)
            })
        } else {
            self.input_ty(db).show_with_db_fmt(f, db, ctx)?;
            f.write_str(" -> ")?;
            self.return_ty(db).show_with_db_fmt(f, db, ctx)
        }
    }
}

#[salsa::tracked(jar = PreciseTermJar)]
pub(crate) fn precise_term_curry_from_raw(
    db: &dyn PreciseTermDb,
    raw_term_curry: RawTermCurry,
) -> PreciseTermResult<PreciseTermCurry> {
    let t =
        |raw_ty| PreciseTerm::from_raw(db, raw_ty, TermTypeExpectation::FinalDestinationEqsSort);
    Ok(PreciseTermCurry::new(
        db,
        raw_term_curry.curry_kind(db),
        raw_term_curry.variance(db),
        match raw_term_curry.input_symbol(db) {
            Some(input_symbol) => Some(PreciseTermSymbol::from_raw(
                db,
                input_symbol,
                TermTypeExpectation::Any,
            )?),
            None => None,
        },
        t(raw_term_curry.input_ty(db))?,
        t(raw_term_curry.return_ty(db))?,
    ))
}

impl<Db: PreciseTermDb + ?Sized> salsa::DisplayWithDb<Db> for PreciseTermCurry {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<PreciseTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl PreciseTermRewriteCopy for PreciseTermCurry {
    fn substitute_copy(
        self,
        db: &dyn PreciseTermDb,
        substituation: &PreciseTermSubstitution,
    ) -> Self {
        todo!()
    }
}
