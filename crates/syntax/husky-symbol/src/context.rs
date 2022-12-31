use crate::*;

use husky_entity_tree::{CratePrelude, EntityTreeDb};
use local_stack::LocalStack;

pub struct SymbolContext<'a> {
    db: &'a dyn EntityTreeDb,
    entity_path: EntityPath,
    crate_prelude: CratePrelude<'a>,
    local_symbol_sheet: &'a mut LocalSymbolSheet,
}

impl<'a> SymbolContext<'a> {
    pub fn new(
        db: &'a dyn EntityTreeDb,
        entity_path: EntityPath,
        crate_prelude: CratePrelude<'a>,
        local_symbol_sheet: &'a mut LocalSymbolSheet,
    ) -> Self {
        Self {
            db,
            entity_path,
            crate_prelude,
            local_symbol_sheet,
        }
    }

    pub fn define_symbol(&mut self, _symbol: Symbol) {
        todo!()
        // self.symbols.push(symbol)
    }

    pub fn resolve_ident(&self, ident: Identifier) -> Option<Symbol> {
        if let Some(symbol) = self.local_symbol_sheet.resolve_ident(ident) {
            Some(symbol)
        } else if let Some(symbol) = self.crate_prelude.resolve_ident(ident) {
            Some(Symbol::Entity(symbol.entity_path()))
        } else {
            None
        }
    }

    pub fn entity_path(&self) -> EntityPath {
        self.entity_path
    }

    pub fn db(&self) -> &dyn EntityTreeDb {
        self.db
    }
}
