use crate::*;
use husky_word::Identifier;
use vec_like::AsVecMapEntry;

#[derive(Default)]
pub struct LocalSymbolSheet {
    variables: Vec<VariableSymbol>,
}

impl LocalSymbolSheet {
    pub fn define_variable(&mut self, _symbol: Symbol) {
        todo!()
        // self.symbols.push(symbol)
    }

    pub fn resolve_ident(&self, ident: Identifier) -> Option<Symbol> {
        // let symbols: Vec<_> = self
        //     .symbols
        //     .iter()
        //     .filter(|entry| entry.symbol.ident == ident)
        //     .collect();
        // if symbols.len() == 0 {
        //     return None;
        // }
        todo!()
    }

    fn try1() {
        let _haha = Haha::default();

        #[derive(Default)]
        struct Haha;
    }
}
