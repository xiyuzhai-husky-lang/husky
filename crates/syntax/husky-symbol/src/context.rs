use crate::*;

use husky_entity_tree::{CratePrelude, EntityTreeDb};
use local_stack::LocalStack;

#[derive(Clone)]
pub struct SymbolContext<'a> {
    db: &'a dyn EntityTreeDb,
    entity_path: EntityPath,
    crate_prelude: CratePrelude<'a>,
    local_symbol_sheet: &'a LocalSymbolSheet,
}

impl<'a> SymbolContext<'a> {
    pub fn new(
        db: &'a dyn EntityTreeDb,
        entity_path: EntityPath,
        crate_prelude: CratePrelude<'a>,
        local_symbol_sheet: &'a LocalSymbolSheet,
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
        todo!()
        // if let Some(symbol) = self.locals.resolve_ident(ident) {
        //     Some(symbol)
        // } else if let Some(symbol) = self.preludes.iter().find(|symbol| symbol.ident == ident) {
        //     Some(*symbol)
        // } else {
        //     None
        // }
    }

    pub fn entity_path(&self) -> EntityPath {
        self.entity_path
    }

    pub fn db(&self) -> &dyn EntityTreeDb {
        self.db
    }
}
