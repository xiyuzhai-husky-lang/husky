use crate::*;
use husky_word::Identifier;
use vec_like::VecMapEntry;

pub struct SymbolSheet {
    symbols: IdentMap<SymbolEntry>,
}

pub struct SymbolEntry {
    symbol: Symbol,
    init_range: TextRange,
    access_range: TextRange,
}

impl VecMapEntry<Identifier> for SymbolEntry {
    fn key(&self) -> Identifier {
        self.symbol.ident
    }

    fn key_ref(&self) -> &Identifier {
        &self.symbol.ident
    }
}

impl SymbolSheet {
    pub(crate) fn new() -> Self {
        Self {
            symbols: Default::default(),
        }
    }

    pub fn define_symbol(&mut self, _symbol: Symbol) {
        todo!()
        // self.symbols.push(symbol)
    }

    pub fn resolve_ident(&self, ident: Identifier) -> Option<Symbol> {
        let symbols: Vec<_> = self
            .symbols
            .iter()
            .filter(|entry| entry.symbol.ident == ident)
            .collect();
        if symbols.len() == 0 {
            return None;
        }
        todo!()
    }

    fn try1() {
        let _haha = Haha::default();

        #[derive(Default)]
        struct Haha;
    }
}
