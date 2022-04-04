use crate::{atom::symbol_proxy::SymbolProxy, *};

impl<'a> AstTransformer<'a> {
    pub(super) fn symbol_proxy(&self) -> SymbolProxy {
        SymbolProxy {
            db: self.db,
            symbols: &self.symbols,
            main: Some(self.main),
            this_ty: self.this.value(),
        }
    }
}
