use super::*;
use thiserror::Error;

#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermSymbol {
    pub ty: RawTermSymbolTypeResult<RawTerm>,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

impl RawTermSymbol {
    pub fn from_precise(db: &dyn RawTermDb, precise_term: RawTermSymbol) -> RawTermResult<Self> {
        todo!()
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

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RawTermSymbolTypeErrorKind {
    #[error("signature valid_term error")]
    SignatureRawTermError,
    #[error("sketch valid_term error")]
    SketchRawTermError,
}
pub type RawTermSymbolTypeResult<T> = Result<T, RawTermSymbolTypeErrorKind>;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct RawTermSymbolRegistry {
    tys: Vec<RawTermSymbolTypeResult<RawTerm>>,
}

impl RawTermSymbolRegistry {
    pub fn new_symbol(
        &mut self,
        db: &dyn RawTermDb,
        ty: RawTermSymbolTypeResult<RawTerm>,
    ) -> RawTermSymbol {
        let idx_usize = self.tys.iter().filter(|ty1| **ty1 == ty).count();
        let idx = match idx_usize.try_into() {
            Ok(idx) => idx,
            Err(_) => todo!(),
        };
        self.tys.push(ty);
        RawTermSymbol::new(db, ty, idx)
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
