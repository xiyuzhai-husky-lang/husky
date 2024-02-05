mod entry;

use self::term::symbol::EthSymbol;
use self::{entry::*, term::rune::EthRune};
use crate::*;
use vec_like::VecMap;

#[derive(Default)]
pub(crate) struct TermShowContext {
    entries: VecMap<TermSymbolShowEntry>,
}

impl TermShowContext {
    pub(crate) fn fmt_symbol(
        &mut self,
        db: &::salsa::Db,
        symbol: EthSymbol,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(entry) = self.entries.get_entry(symbol) {
            entry.show(db, f)
        } else {
            let new_entry = self.new_external_entry(db, symbol, None);
            new_entry.show(db, f)?;
            self.entries.insert_new(new_entry).unwrap();
            Ok(())
        }
    }

    pub(crate) fn fmt_with_variable(
        &mut self,
        _db: &::salsa::Db,
        _variable: EthRune,
        f: impl FnOnce(&mut Self) -> std::fmt::Result,
    ) -> std::fmt::Result {
        // ad hoc
        f(self)
    }
}
