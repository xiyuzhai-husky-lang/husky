mod registry;
mod set;

pub use self::registry::*;
pub use self::set::*;

use super::*;
use thiserror::Error;
use vec_like::VecSet;

#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermSymbol {
    pub qualified_ty: RawTermSymbolTypeResult<RawTermQualifiedType>,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

impl RawTermSymbol {
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
    #[error("signature raw_term error")]
    SignatureRawTermError,
    #[error("sketch raw_term error")]
    SketchRawTermError,
}
pub type RawTermSymbolTypeResult<T> = Result<T, RawTermSymbolTypeErrorKind>;

impl<Db: RawTermDb + ?Sized> salsa::DisplayWithDb<Db> for RawTermSymbol {
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
