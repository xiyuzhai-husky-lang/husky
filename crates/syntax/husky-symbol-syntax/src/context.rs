use crate::*;

use local_stack::LocalStack;

pub type SymbolStack<'a> = LocalStack<Symbol, &'a [Symbol]>;

pub struct SymbolContext<'a> {
    preludes: &'a [Symbol],
    symbols: SymbolSheet,
}

impl<'a> SymbolContext<'a> {
    pub(crate) fn new(preludes: &'a [Symbol]) -> Self {
        Self {
            preludes,
            symbols: SymbolSheet::new(),
        }
    }

    pub fn define_symbol(&mut self, _symbol: Symbol) {
        todo!()
        // self.symbols.push(symbol)
    }

    pub fn resolve_ident(&self, ident: Identifier) -> Option<Symbol> {
        if let Some(symbol) = self.symbols.resolve_ident(ident) {
            Some(symbol)
        } else if let Some(symbol) = self.preludes.iter().find(|symbol| symbol.ident == ident) {
            Some(*symbol)
        } else {
            None
        }
    }
}
