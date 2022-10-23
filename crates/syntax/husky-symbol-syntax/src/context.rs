use crate::*;
use local_stack::LocalStack;

pub type SymbolStack<'a> = LocalStack<Symbol, &'a [Symbol]>;

pub struct SymbolContext<'a> {
    db: &'a dyn SymbolDb,
    symbols: SymbolStack<'a>,
}

impl<'a> SymbolContext<'a> {
    pub fn new(db: &'a dyn SymbolDb, prelude_symbols: &'a [Symbol]) -> Self {
        Self {
            db,
            symbols: LocalStack::new(prelude_symbols),
        }
    }

    pub fn define_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol)
    }

    pub fn resolve_ident(&self, ident: Identifier) -> Symbol {
        if let Some(symbol) = self.symbols.find_last(|symbol| symbol.ident == ident) {
            *symbol
        } else {
            // ad hoc
            Symbol {
                ident,
                kind: SymbolKind::Unrecognized,
            }
        }
    }
}
