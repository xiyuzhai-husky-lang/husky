use super::*;
use context::*;

/// representing valid_term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = ValidTermDb, jar = ValidTermJar)]
pub struct ValidTermCurry {
    pub curry_kind: ValidCurryKind,
    pub variance: Variance,
    /// a
    pub parameter_symbol: Option<ValidTermSymbol>,
    /// X
    pub parameter_ty: ValidTerm,
    /// Y
    pub return_ty: ValidTerm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidCurryKind {
    Explicit,
    Implicit,
}

impl ValidTermCurry {
    pub fn from_precise(
        db: &dyn ValidTermDb,
        precise_term: PreciseTermCurry,
    ) -> ValidTermResult<Self> {
        todo!()
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
    fn substitute_copy(self, db: &dyn ValidTermDb, substituation: &ValidTermSubstitution) -> Self {
        todo!()
    }
}
