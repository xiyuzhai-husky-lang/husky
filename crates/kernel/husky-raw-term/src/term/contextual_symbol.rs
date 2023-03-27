use super::*;
use thiserror::Error;

#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermConcreteSymbol {
    pub ty: RawTermSymbolTypeResult<RawTerm>,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

impl RawTermConcreteSymbol {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        ctx.fmt_contextual_symbol(db, self, f)
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RawTermSymbolTypeErrorKind {
    #[error("signature raw_term error")]
    SignatureRawTermError,
    #[error("sketch raw_term error")]
    SketchRawTermError,
}
pub type RawTermSymbolTypeResult<T> = Result<T, RawTermSymbolTypeErrorKind>;

// todo: change to ty_final_destinations: Vec<RawTermSymbolTypeResult<TypeFinalDestination>>,
// RawTerm won't work
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct TermSymbolRegistry {
    ty_final_destinations: Vec<RawTermSymbolTypeResult<RawTerm>>,
}

impl TermSymbolRegistry {
    pub fn new_symbol(
        &mut self,
        db: &dyn RawTermDb,
        ty: RawTermSymbolTypeResult<RawTerm>,
    ) -> RawTermConcreteSymbol {
        let idx_usize = self
            .ty_final_destinations
            .iter()
            .filter(|ty1| **ty1 == ty)
            .count();
        let idx = match idx_usize.try_into() {
            Ok(idx) => idx,
            Err(_) => todo!(),
        };
        self.ty_final_destinations.push(ty);
        RawTermConcreteSymbol::new(db, ty, idx)
    }
}

impl<Db: RawTermDb + ?Sized> salsa::DisplayWithDb<Db> for RawTermConcreteSymbol {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<RawTermJar>>::as_jar_db(db);
        f.write_fmt(format_args!("${}", self.idx(db)))
    }
}
