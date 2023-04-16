mod registry;
mod set;

pub use self::registry::*;
pub use self::set::*;

use super::*;
use thiserror::Error;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar, constructor = new_inner)]
pub struct EtherealTermSymbol {
    pub ty: EtherealTerm,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

#[test]
fn term_symbol_size_works() {
    assert_eq!(
        std::mem::size_of::<EtherealTermSymbol>(),
        std::mem::size_of::<u32>()
    );
}

impl EtherealTermSymbol {
    #[inline(always)]
    pub fn from_raw(
        db: &dyn EtherealTermDb,
        raw_term_symbol: DeclarativeTermSymbol,
    ) -> TermResult<Self> {
        Self::from_raw_unchecked(db, raw_term_symbol)
    }

    #[inline(always)]
    pub(crate) fn from_raw_unchecked(
        db: &dyn EtherealTermDb,
        raw_term_symbol: DeclarativeTermSymbol,
    ) -> TermResult<Self> {
        let ty = raw_term_symbol.ty(db)?;
        let ty = EtherealTerm::ty_from_raw_unchecked(db, ty)?;
        Ok(Self::new_inner(db, ty, raw_term_symbol.idx(db)))
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EtherealTermDb,
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

impl<Db: EtherealTermDb + ?Sized> salsa::DisplayWithDb<Db> for EtherealTermSymbol {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EtherealTermJar>>::as_jar_db(db);
        f.write_fmt(format_args!("${}", self.idx(db)))
    }
}
