mod symbol;

use crate::*;
use symbol::*;

use vec_like::VecMap;

#[derive(Default)]
pub(crate) struct DeclarativeTermShowContext {
    entries: VecMap<DeclarativeTermSymbolShowEntry>,
}

impl DeclarativeTermShowContext {
    pub(crate) fn fmt_symbol(
        &mut self,
        db: &dyn DeclarativeTermDb,
        symbol: DeclarativeTermSymbol,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(entry) = self.entries.get_entry(symbol) {
            entry.show(db, f)
        } else {
            let new_entry = self.new_external_entry(db, symbol, None);
            new_entry.show(db, f);
            self.entries.insert_new(new_entry).unwrap();
            Ok(())
        }
    }

    pub(crate) fn fmt_with_symbol(
        &mut self,
        db: &dyn DeclarativeTermDb,
        symbol: DeclarativeTermSymbol,
        f: impl FnOnce(&mut Self) -> std::fmt::Result,
    ) -> std::fmt::Result {
        self.with_symbol(db, symbol);
        f(self)?;
        self.without_symbol(symbol);
        Ok(())
    }

    pub(crate) fn fmt_variable(
        &mut self,
        db: &dyn DeclarativeTermDb,
        variable: DeclarativeTermRune,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        // ad hoc
        f.write_fmt(format_args!("v{}", variable.idx(db)))
    }

    pub(crate) fn fmt_with_variable(
        &mut self,
        db: &dyn DeclarativeTermDb,
        variable: DeclarativeTermRune,
        f: impl FnOnce(&mut Self) -> std::fmt::Result,
    ) -> std::fmt::Result {
        // ad hoc
        f(self)
    }
}
