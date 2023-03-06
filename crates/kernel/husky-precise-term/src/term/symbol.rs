use super::*;
use thiserror::Error;

#[salsa::interned(db = RawTermDb, jar = RawTermJar, constructor = new_inner)]
pub struct RawTermSymbol {
    pub ty: RawTerm,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

impl RawTermSymbol {
    pub fn from_raw(
        db: &dyn RawTermDb,
        raw_term: RawTermSymbol,
        raw_ty_expectation: TermTypeExpectation,
    ) -> RawTermResult<Self> {
        let ty = raw_term.ty(db)?;
        let ty = RawTerm::from_raw(db, ty, TermTypeExpectation::FinalDestinationEqsSort)?;
        Ok(Self::new_inner(db, ty, raw_term.idx(db)))
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        ctx.fmt_symbol(db, self, f)
    }
}

impl<Db: RawTermDb + ?Sized> salsa::DisplayWithDb<Db> for RawTermSymbol {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<RawTermJar>>::as_jar_db(db);
        f.write_fmt(format_args!("${}", self.idx(db)))
    }
}
