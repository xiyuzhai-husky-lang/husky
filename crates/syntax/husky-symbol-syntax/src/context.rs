use crate::*;
use local_stack::LocalStack;

pub struct SymbolContext<'a> {
    db: &'a dyn SymbolDb,
    symbols: LocalStack<Symbol>,
}

impl<'a> SymbolContext<'a> {
    pub fn new(db: &'a dyn SymbolDb) -> Self {
        Self {
            db,
            symbols: Default::default(),
        }
    }

    pub fn define_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol)
    }

    pub fn resolve_ident(&self, ident: Identifier) -> Symbol {
        // ad hoc
        Symbol {
            ident,
            kind: SymbolKind::Unrecognized,
        }
    }
}
