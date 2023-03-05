use super::*;
use thiserror::Error;

#[salsa::interned(db = PreciseTermDb, jar = PreciseTermJar)]
pub struct PreciseTermSymbol {
    pub ty: PreciseTermSymbolTypeResult<PreciseTerm>,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

impl PreciseTermSymbol {
    pub fn from_raw(
        db: &dyn PreciseTermDb,
        raw_term: RawTermSymbol,
        raw_ty_expectation: TypeExpectation,
    ) -> PreciseTermResult<Self> {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn PreciseTermDb,
        ctx: &mut PreciseTermShowContext,
    ) -> std::fmt::Result {
        ctx.fmt_symbol(db, self, f)
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PreciseTermSymbolTypeErrorKind {
    #[error("signature precise_term error")]
    SignaturePreciseTermError,
    #[error("sketch precise_term error")]
    SketchPreciseTermError,
}
pub type PreciseTermSymbolTypeResult<T> = Result<T, PreciseTermSymbolTypeErrorKind>;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct PreciseTermSymbolRegistry {
    tys: Vec<PreciseTermSymbolTypeResult<PreciseTerm>>,
}

impl PreciseTermSymbolRegistry {
    pub fn new_symbol(
        &mut self,
        db: &dyn PreciseTermDb,
        ty: PreciseTermSymbolTypeResult<PreciseTerm>,
    ) -> PreciseTermSymbol {
        let idx_usize = self.tys.iter().filter(|ty1| **ty1 == ty).count();
        let idx = match idx_usize.try_into() {
            Ok(idx) => idx,
            Err(_) => todo!(),
        };
        self.tys.push(ty);
        PreciseTermSymbol::new(db, ty, idx)
    }
}

impl<Db: PreciseTermDb + ?Sized> salsa::DisplayWithDb<Db> for PreciseTermSymbol {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<PreciseTermJar>>::as_jar_db(db);
        f.write_fmt(format_args!("${}", self.idx(db)))
    }
}
