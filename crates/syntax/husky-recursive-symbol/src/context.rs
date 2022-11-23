use crate::*;
use husky_entity_tree::EntityTreeDb;
use local_stack::LocalStack;

pub struct RecursiveSymbolContext<'a> {
    preludes: &'a [RecursiveSymbol],
    symbols: &'a mut RecursiveSymbolSheet,
}

impl<'a> RecursiveSymbolContext<'a> {
    pub fn new(preludes: &'a [RecursiveSymbol], symbols: &'a mut RecursiveSymbolSheet) -> Self {
        Self { preludes, symbols }
    }

    pub fn define_symbol(&mut self, symbol: RecursiveSymbol) {
        todo!()
        // self.symbols.push(symbol)
    }

    pub fn resolve_ident(&self, ident: Identifier) -> RecursiveSymbol {
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
