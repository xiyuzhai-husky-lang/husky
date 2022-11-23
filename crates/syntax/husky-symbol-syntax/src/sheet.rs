use crate::*;
use husky_identifier::Identifier;
use vec_like::VecMapEntry;

pub struct SymbolSheet {
    data: IdentMap<Symbol>,
}

impl VecMapEntry<Identifier> for Symbol {
    fn key(&self) -> Identifier {
        self.ident
    }
}

impl SymbolSheet {
    pub(crate) fn new() -> Self {
        Self {
            data: Default::default(),
        }
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

    fn try1() {
        let haha = Haha::default();

        #[derive(Default)]
        struct Haha;
    }
}
