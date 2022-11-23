use crate::*;
use husky_identifier::Identifier;

#[derive(Default)]
pub struct SymbolSheet {}

impl SymbolSheet {
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
