mod registry;
mod set;

pub use self::registry::*;
pub use self::set::*;

use super::*;
use thiserror::Error;

#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermSymbol {
    pub ty: TermSymbolTypeResult<Term>,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

impl TermSymbol {
    #[inline(always)]
    pub fn from_raw(db: &dyn TermDb, valid_term: RawTermSymbol) -> TermResult<Self> {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        ctx.fmt_symbol(db, self, f)
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermSymbolTypeErrorKind {
    #[error("signature term error")]
    SignatureTermError,
    #[error("sketch term error")]
    SketchTermError,
}
pub type TermSymbolTypeResult<T> = Result<T, TermSymbolTypeErrorKind>;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct TermSymbolRegistry {
    tys: Vec<TermSymbolTypeResult<Term>>,
}

impl TermSymbolRegistry {
    pub fn new_symbol(&mut self, db: &dyn TermDb, ty: TermSymbolTypeResult<Term>) -> TermSymbol {
        let idx_usize = self.tys.iter().filter(|ty1| **ty1 == ty).count();
        let idx = match idx_usize.try_into() {
            Ok(idx) => idx,
            Err(_) => todo!(),
        };
        self.tys.push(ty);
        TermSymbol::new(db, ty, idx)
    }
}

impl<Db: TermDb + ?Sized> salsa::DisplayWithDb<Db> for TermSymbol {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermJar>>::as_jar_db(db);
        f.write_fmt(format_args!("${}", self.idx(db)))
    }
}
