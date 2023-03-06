use super::*;
use thiserror::Error;

#[salsa::interned(db = ValidTermDb, jar = ValidTermJar)]
pub struct ValidTermSymbol {
    pub ty: ValidTermSymbolTypeResult<ValidTerm>,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

impl ValidTermSymbol {
    pub fn from_precise(
        db: &dyn ValidTermDb,
        precise_term: PreciseTermSymbol,
    ) -> ValidTermResult<Self> {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn ValidTermDb,
        ctx: &mut ValidTermShowContext,
    ) -> std::fmt::Result {
        ctx.fmt_symbol(db, self, f)
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ValidTermSymbolTypeErrorKind {
    #[error("signature valid_term error")]
    SignatureValidTermError,
    #[error("sketch valid_term error")]
    SketchValidTermError,
}
pub type ValidTermSymbolTypeResult<T> = Result<T, ValidTermSymbolTypeErrorKind>;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ValidTermSymbolRegistry {
    tys: Vec<ValidTermSymbolTypeResult<ValidTerm>>,
}

impl ValidTermSymbolRegistry {
    pub fn new_symbol(
        &mut self,
        db: &dyn ValidTermDb,
        ty: ValidTermSymbolTypeResult<ValidTerm>,
    ) -> ValidTermSymbol {
        let idx_usize = self.tys.iter().filter(|ty1| **ty1 == ty).count();
        let idx = match idx_usize.try_into() {
            Ok(idx) => idx,
            Err(_) => todo!(),
        };
        self.tys.push(ty);
        ValidTermSymbol::new(db, ty, idx)
    }
}

impl<Db: ValidTermDb + ?Sized> salsa::DisplayWithDb<Db> for ValidTermSymbol {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<ValidTermJar>>::as_jar_db(db);
        f.write_fmt(format_args!("${}", self.idx(db)))
    }
}
