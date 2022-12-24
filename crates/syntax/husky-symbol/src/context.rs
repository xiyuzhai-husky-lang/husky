use crate::*;

use husky_entity_tree::EntityTreeDb;
use local_stack::LocalStack;

#[derive(Clone)]
pub struct SymbolContext<'a> {
    sheet: &'a LocalSymbolSheet,
}

impl<'a> SymbolContext<'a> {
    pub fn new(db: &dyn EntityTreeDb, preludes: &'a [Symbol], locals: &'a [Symbol]) -> Self {
        todo!()
        // Self { preludes, locals }
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
}
