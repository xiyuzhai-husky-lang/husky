use super::*;
use context::*;

/// representing valid_term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = ValidTermDb, jar = ValidTermJar, constructor = new_inner)]
pub struct ValidTermCurry {
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub parameter_symbol: Option<ValidTermSymbol>,
    /// X
    pub parameter_ty: ValidTerm,
    /// Y
    pub return_ty: ValidTerm,
}

impl ValidTermCurry {
    pub fn from_precise(
        db: &dyn ValidTermDb,
        precise_term_curry: PreciseTermCurry,
    ) -> ValidTermResult<Self> {
        valid_term_curry_from_precise(db, precise_term_curry)
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn ValidTermDb,
        ctx: &mut ValidTermShowContext,
    ) -> std::fmt::Result {
        let parameter_symbol = self.parameter_symbol(db);
        if parameter_symbol.is_some() {
            f.write_str("(")?
        }
        f.write_str(self.variance(db).as_str())?;
        if let Some(parameter_symbol) = parameter_symbol {
            ctx.fmt_with_symbol(db, parameter_symbol, |ctx| {
                ctx.fmt_symbol(db, parameter_symbol, f);
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

#[salsa::tracked(jar = ValidTermJar)]
pub(crate) fn valid_term_curry_from_precise(
    db: &dyn ValidTermDb,
    precise_term_curry: PreciseTermCurry,
) -> ValidTermResult<ValidTermCurry> {
    let curry_kind = precise_term_curry.curry_kind(db);
    let variance = precise_term_curry.variance(db);
    let parameter_symbol = match precise_term_curry.parameter_symbol(db) {
        Some(parameter_symbol) => Some(ValidTermSymbol::from_precise(db, parameter_symbol)?),
        None => None,
    };
    let t = |precise_term| ValidTerm::from_precise(db, precise_term);
    let parameter_ty = t(precise_term_curry.parameter_ty(db))?;
    let return_ty = t(precise_term_curry.return_ty(db))?;
    Ok(ValidTermCurry::new_inner(
        db,
        curry_kind,
        variance,
        parameter_symbol,
        parameter_ty,
        return_ty,
    ))
}

fn check_curry_validity(
    db: &dyn ValidTermDb,
    parameter_ty: ValidTerm,
    return_ty: ValidTerm,
) -> ValidTermResult<()> {
    if !parameter_ty.is_ins_sort(db)? {
        return Err(todo!());
    }
    if !return_ty.is_ins_sort(db)? {
        return Err(todo!());
    }
    Ok(())
}

impl<Db: ValidTermDb + ?Sized> salsa::DisplayWithDb<Db> for ValidTermCurry {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<ValidTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl ValidTermRewriteCopy for ValidTermCurry {
    fn substitute(self, db: &dyn ValidTermDb, substituation: &ValidTermSubstitution) -> Self {
        todo!()
    }
}
