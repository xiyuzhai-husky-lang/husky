use super::*;
use husky_symbol::Symbol;

pub struct SymbolStack<'a> {
    crate_prelude: CratePrelude<'a>,
}

impl<'a> SymbolStack<'a> {
    pub fn new(crate_prelude: CratePrelude<'a>) -> Self {
        Self { crate_prelude }
    }

    pub(crate) fn resolve_ident(&self, ident: Identifier) -> Option<Symbol> {
        // ad hoc
        if let Some(symbol) = self.crate_prelude.resolve_ident(ident) {
            return Some(Symbol::Entity(symbol.entity_path()));
        } else {
            None
        }
    }
}
