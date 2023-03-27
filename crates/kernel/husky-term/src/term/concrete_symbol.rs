mod registry;
mod set;

pub use self::registry::*;
pub use self::set::*;

use super::*;
use thiserror::Error;

#[salsa::interned(db = TermDb, jar = TermJar, constructor = new_inner)]
pub struct TermConcreteSymbol {
    pub ty: Term,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

#[test]
fn term_symbol_size_works() {
    assert_eq!(
        std::mem::size_of::<TermConcreteSymbol>(),
        std::mem::size_of::<u32>()
    );
}

impl TermConcreteSymbol {
    #[inline(always)]
    pub(crate) fn from_raw_unchecked(
        db: &dyn TermDb,
        raw_term_symbol: RawTermConcreteSymbol,
    ) -> TermResult<Self> {
        let ty = raw_term_symbol.ty(db)?;
        let ty = Term::from_raw_unchecked(db, ty, TermTypeExpectation::FinalDestinationEqsSort)?;
        Ok(Self::new_inner(db, ty, raw_term_symbol.idx(db)))
    }

    pub(super) fn check(self, db: &dyn TermDb) -> TermResult<()> {
        self.ty(db).check(db)
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
    // todo: change to final destination
    tys: Vec<Term>,
}

impl TermSymbolRegistry {
    pub fn new_symbol(&mut self, db: &dyn TermDb, ty: Term) -> TermConcreteSymbol {
        let idx_usize = self.tys.iter().filter(|ty1| **ty1 == ty).count();
        let idx = match idx_usize.try_into() {
            Ok(idx) => idx,
            Err(_) => todo!(),
        };
        self.tys.push(ty);
        TermConcreteSymbol::new_inner(db, ty, idx)
    }
}

impl<Db: TermDb + ?Sized> salsa::DisplayWithDb<Db> for TermConcreteSymbol {
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
