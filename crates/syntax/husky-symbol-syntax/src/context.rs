use crate::*;
use husky_entity_tree::EntityTreeDb;
use local_stack::LocalStack;

pub type SymbolStack<'a> = LocalStack<Symbol, &'a [Symbol]>;

pub struct SymbolContext<'a> {
    preludes: &'a [Symbol],
    symbols: &'a mut SymbolSheet,
}

impl<'a> SymbolContext<'a> {
    pub fn new(preludes: &'a [Symbol], symbols: &'a mut SymbolSheet) -> Self {
        Self { preludes, symbols }
    }

    pub fn define_symbol(&mut self, symbol: Symbol) {
        todo!()
        // self.symbols.push(symbol)
    }

    pub fn resolve_ident(&self, ident: Identifier) -> Symbol {
        todo!()
        // if let Some(symbol) = self.symbols.find_last(|symbol| symbol.ident == ident) {
        //     *symbol
        // } else {
        //     // ad hoc
        //     Symbol {
        //         ident,
        //         kind: SymbolKind::Unrecognized,
        //     }
        // }
    }
}
