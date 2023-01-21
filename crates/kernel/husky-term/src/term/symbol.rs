use crate::*;
use thiserror::Error;

#[salsa::interned(jar = TermJar)]
pub struct TermSymbol {
    ty: TermSymbolTypeResult<Term>,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    idx: u8,
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermSymbolTypeErrorKind {}
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
