use super::*;
use context::*;

/// representing valid_term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = ValidTermDb, jar = ValidTermJar)]
pub struct ValidTermCurry {
    pub curry_kind: ValidCurryKind,
    pub variance: Variance,
    /// a
    pub input_symbol: Option<ValidTermSymbol>,
    /// X
    pub input_ty: ValidTerm,
    /// Y
    pub return_ty: ValidTerm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidCurryKind {
    Explicit,
    Implicit,
}

impl ValidTermCurry {
    pub fn from_precise(db: &dyn ValidTermDb, precise_term: PreciseTermCurry) -> Self {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn ValidTermDb,
        ctx: &mut ValidTermShowContext,
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
