use crate::*;
use husky_identifier::Identifier;

#[derive(Default)]
pub struct RecursiveSymbolSheet {}

impl RecursiveSymbolSheet {
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

    fn try1() {
        let haha = Haha::default();

        #[derive(Default)]
        struct Haha;
    }
}
