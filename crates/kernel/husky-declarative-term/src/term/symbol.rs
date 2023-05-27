mod registry;
mod set;

pub use self::registry::*;
pub use self::set::*;

use super::*;
use thiserror::Error;
use vec_like::VecSet;

#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermSymbol {
    pub ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

impl DeclarativeTermSymbol {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn DeclarativeTermDb,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        ctx.fmt_symbol(db, self, f)
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DeclarativeTermSymbolTypeErrorKind {
    #[error("signature declarative_term error")]
    SignatureDeclarativeTermError,
    #[error("sketch declarative_term error")]
    SketchDeclarativeTermError,
}
pub type DeclarativeTermSymbolTypeResult<T> = Result<T, DeclarativeTermSymbolTypeErrorKind>;

impl<Db: DeclarativeTermDb + ?Sized> salsa::DisplayWithDb<Db> for DeclarativeTermSymbol {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclarativeTermJar>>::as_jar_db(db);
        f.write_fmt(format_args!("${}", self.idx(db)))
    }
}
