mod registry;
mod set;

pub use self::registry::*;
pub use self::set::*;

use super::*;
use thiserror::Error;

#[salsa::interned(db = TermDb, jar = TermJar, constructor = new_inner)]
pub struct TermSymbol {
    pub qualified_ty: TermQualifiedType,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

#[test]
fn term_symbol_size_works() {
    assert_eq!(
        std::mem::size_of::<TermSymbol>(),
        std::mem::size_of::<u32>()
    );
}

impl TermSymbol {
    #[inline(always)]
    pub(crate) fn from_raw_unchecked(
        db: &dyn TermDb,
        raw_term_symbol: RawTermSymbol,
    ) -> TermResult<Self> {
        let qualified_ty = raw_term_symbol.qualified_ty(db)?;
        let ty = TermQualifiedType::from_raw_unchecked(db, qualified_ty)?;
        Ok(Self::new_inner(db, ty, raw_term_symbol.idx(db)))
    }

    pub(super) fn check(self, db: &dyn TermDb) -> TermResult<()> {
        self.qualified_ty(db).check(db)
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
