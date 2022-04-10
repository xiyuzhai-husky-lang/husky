use crate::*;
use atom::symbol_proxy::SymbolProxy;

impl<'a> AstTransformer<'a> {
    pub(super) fn symbol_proxy(&self) -> SymbolProxy {
        SymbolProxy {
            db: self.db.upcast(),
            symbols: &self.symbols,
            opt_package_main: Some(self.main),
            opt_this_ty: self.this.value(),
        }
    }
}
